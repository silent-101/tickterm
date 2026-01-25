#![allow(non_snake_case)]

use std::{sync::mpsc, thread, time::Duration};

use crate::tickterm::components::{
    clock::{ui::TimeUI, utils::Time},
    stopwatch::{
        state::{BtnMode, BtnState},
        ui::StopWatchUI,
        utils::StopWatch,
    },
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

#[derive(Debug, Default)]
pub struct TickTerm {
    is_running: bool,
    mode_stop_watch: bool,
    mode_clock: bool,
    toggle_key_binds: bool,
}

impl TickTerm {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn runner(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.is_running = true;
        self.mode_clock = true;
        self.toggle_key_binds = true;
        self.mode_stop_watch = false;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                if let Ok(Event::Key(key)) = event::read()
                    && tx.send(key).is_err()
                {
                    break; // Main thread exited, stop listening
                }
            }
        });
        let mut stop_watch_btn: BtnState = BtnState::new();
        let mut watch = StopWatch::default();
        while self.is_running {
            if self.mode_clock {
                let mut current_time = Time::default();
                current_time.set_new_time();
                terminal.draw(|frame| {
                    self.dom(frame, Some(&current_time), None, None);
                })?;
            } else if !self.mode_clock && self.mode_stop_watch {
                watch.update();
                terminal.draw(|frame| {
                    self.dom(frame, None, Some(&watch), Some(&stop_watch_btn.get_state()));
                })?;
            }
            match rx.recv_timeout(Duration::from_millis(if self.mode_stop_watch {
                if watch.is_running() { 10 } else { 500 }
            } else {
                500
            })) {
                Ok(key) => {
                    if KeyCode::Esc == key.code {
                        self.is_running = false;
                    } else if KeyCode::Char('w') == key.code {
                        self.mode_clock = false;
                        self.mode_stop_watch = true;
                    } else if KeyCode::Char('b') == key.code {
                        self.mode_clock = true;
                        self.mode_stop_watch = false;
                    } else if KeyCode::Char('i') == key.code {
                        self.toggle_key_binds = !self.toggle_key_binds
                    } else if KeyCode::Tab == key.code && self.mode_stop_watch {
                        match (stop_watch_btn.get_state(), watch.is_running()) {
                            (BtnMode::Start, true) => stop_watch_btn.change_state(BtnMode::Stop),
                            (BtnMode::Stop, true) => stop_watch_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, true) => stop_watch_btn.change_state(BtnMode::Stop),
                            (BtnMode::Start, false) => stop_watch_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, false) => stop_watch_btn.change_state(BtnMode::Start),
                            (BtnMode::Stop, false) => stop_watch_btn.change_state(BtnMode::Start),
                        }
                    } else if KeyCode::Enter == key.code && self.mode_stop_watch {
                        match stop_watch_btn.get_state() {
                            BtnMode::Start => {
                                watch.start();
                                stop_watch_btn.change_state(BtnMode::Stop);
                            }
                            BtnMode::Reset => {
                                watch.reset();
                                stop_watch_btn.change_state(BtnMode::Start);
                            }
                            BtnMode::Stop => {
                                watch.stop();
                                stop_watch_btn.change_state(BtnMode::Start);
                            }
                        }
                    }
                }
                Err(_) => {
                    // Timeout - continue to next frame
                }
            }
        }
        Ok(())
    }

    fn dom(
        &mut self,
        frame: &mut Frame,
        t: Option<&Time>,
        w: Option<&StopWatch>,
        btn: Option<&BtnMode>,
    ) {
        // Minimum required size
        let min_width = 76;
        let min_height = 10;

        // Check if terminal is too small
        if frame.area().width < min_width || frame.area().height < min_height {
            let message = Paragraph::new(vec![
                Line::from(Span::styled("Terminal too small!", Color::Red)),
                Line::from(""),
                Line::from(format!("Minimum size: {}x{}", min_width, min_height)),
                Line::from(format!(
                    "Current size: {}x{}",
                    frame.area().width,
                    frame.area().height
                )),
            ])
            .alignment(Alignment::Center)
            .block(Block::default());

            let centered = frame
                .area()
                .centered(Constraint::Length(30), Constraint::Length(4));
            frame.render_widget(message, centered);
            return;
        }

        let key_binds: Option<Paragraph<'static>> = {
            if self.toggle_key_binds {
                let info = {
                    if self.mode_clock {
                        "<Esc>Quit | <w>Stopwatch | <i>Toggle-KeyBinds"
                    } else {
                        "<Esc>Quit | <b>Back | <Tab>Switch-Button | <Enter>Click | <i>Toggle-KeyBinds"
                    }
                };
                Some(
                    Paragraph::new(Line::from(Span::raw(info)))
                        .alignment(ratatui::layout::Alignment::Center)
                        .fg(Color::Rgb(75, 75, 75))
                        .bold(),
                )
            } else {
                None
            }
        };
        frame.render_widget(
            key_binds,
            frame
                .area()
                .centered(Constraint::Percentage(100), Constraint::Percentage(95)),
        );
        if self.mode_clock {
            if let Some(t) = t {
                TimeUI(frame, t);
            }
        } else if self.mode_stop_watch
            && let Some(w) = w
            && let Some(btn) = btn
        {
            StopWatchUI(frame, w, btn);
        }
    }
}
