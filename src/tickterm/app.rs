#![allow(non_snake_case)]

use std::{sync::mpsc, thread, time::Duration};

use crate::tickterm::components::{
    clock::{ui::TimeUI, utils::Time},
    stopwatch::{
        state::{BtnMode, BtnState},
        ui::StopWatchUI,
        utils::StopWatch,
    },
    timer::{
        state::{TimerCtx, TimerMode, TimerSagemnt},
        ui::TimerUI,
        utils::TimerUtils,
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

#[derive(Debug, Default, PartialEq, Eq)]
enum TickTermMode {
    #[default]
    Normal,
    StopWatch,
    Timer,
}

#[derive(Debug, Default)]
pub struct TickTerm {
    is_running: bool,
    mode: TickTermMode,
    toggle_key_binds: bool,
}

impl TickTerm {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn runner(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.is_running = true;
        self.mode = TickTermMode::Normal;
        self.toggle_key_binds = true;
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
        let mut timer_btn: BtnState = BtnState::new();
        timer_btn.change_state(BtnMode::Set);
        let mut watch = StopWatch::default();
        let mut timer_ctx = TimerCtx::new();
        let mut timer_utils = TimerUtils::new();
        while self.is_running {
            match self.mode {
                TickTermMode::Normal => {
                    let mut current_time = Time::default();
                    current_time.set_new_time();
                    terminal.draw(|frame| {
                        self.dom(frame, Some(&current_time), None, None, None, None);
                    })?;
                }
                TickTermMode::StopWatch => {
                    watch.update();
                    terminal.draw(|frame| {
                        self.dom(
                            frame,
                            None,
                            Some(&watch),
                            Some(&stop_watch_btn.get_state()),
                            None,
                            None,
                        );
                    })?;
                }
                TickTermMode::Timer => {
                    // Check if timer just finished
                    if !timer_utils.is_timer_running()
                        && timer_ctx.get_timer_mode() == &TimerMode::RunningMode
                    {
                        timer_utils.reset();
                        timer_ctx.set_timer_mode(TimerMode::NormalMode);
                        timer_btn.change_state(BtnMode::Set);
                    }
                    terminal.draw(|frame| {
                        self.dom(
                            frame,
                            None,
                            None,
                            Some(&timer_btn.get_state()),
                            Some(&timer_ctx),
                            Some(&timer_utils),
                        );
                    })?;
                }
            }

            match rx.recv_timeout(Duration::from_millis(
                if self.mode == TickTermMode::StopWatch {
                    if watch.is_running() { 10 } else { 500 }
                } else if self.mode == TickTermMode::Timer && timer_utils.is_timer_running() {
                    100
                } else {
                    500
                },
            )) {
                Ok(key) => {
                    if KeyCode::Esc == key.code {
                        self.is_running = false;
                    } else if KeyCode::Char('w') == key.code {
                        self.mode = TickTermMode::StopWatch
                    } else if KeyCode::Char('b') == key.code {
                        self.mode = TickTermMode::Normal
                    } else if KeyCode::Char('i') == key.code {
                        self.toggle_key_binds = !self.toggle_key_binds
                    } else if KeyCode::Char('t') == key.code {
                        self.mode = TickTermMode::Timer
                    } else if KeyCode::Tab == key.code && self.mode == TickTermMode::StopWatch {
                        match (stop_watch_btn.get_state(), watch.is_running()) {
                            (BtnMode::Start, true) => stop_watch_btn.change_state(BtnMode::Stop),
                            (BtnMode::Stop, true) => stop_watch_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, true) => stop_watch_btn.change_state(BtnMode::Stop),
                            (BtnMode::Set, true) => stop_watch_btn.change_state(BtnMode::Stop),
                            (BtnMode::Start, false) => stop_watch_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, false) => stop_watch_btn.change_state(BtnMode::Start),
                            (BtnMode::Stop, false) => stop_watch_btn.change_state(BtnMode::Start),
                            (BtnMode::Set, false) => stop_watch_btn.change_state(BtnMode::Start),
                        }
                    } else if KeyCode::Tab == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() != &TimerMode::SetterMode
                    {
                        let has_value = timer_utils.has_value();
                        match (
                            timer_btn.get_state(),
                            timer_ctx.get_timer_mode() == &TimerMode::RunningMode,
                            has_value,
                        ) {
                            (BtnMode::Start, true, _) => timer_btn.change_state(BtnMode::Stop),
                            (BtnMode::Stop, true, _) => timer_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, true, _) => timer_btn.change_state(BtnMode::Stop),
                            (BtnMode::Set, true, _) => timer_btn.change_state(BtnMode::Stop),
                            (BtnMode::Start, false, true) => timer_btn.change_state(BtnMode::Reset),
                            (BtnMode::Set, false, false) => timer_btn.change_state(BtnMode::Reset),
                            (BtnMode::Reset, false, true) => timer_btn.change_state(BtnMode::Start),
                            (BtnMode::Reset, false, false) => timer_btn.change_state(BtnMode::Set),
                            (BtnMode::Stop, false, _) => timer_btn.change_state(BtnMode::Start),
                            _ => {}
                        }
                    } else if KeyCode::Enter == key.code && self.mode == TickTermMode::StopWatch {
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
                            BtnMode::Set => {
                                timer_ctx.set_timer_mode(TimerMode::SetterMode);
                            }
                        }
                    } else if KeyCode::Enter == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() != &TimerMode::SetterMode
                    {
                        match timer_btn.get_state() {
                            BtnMode::Start => {
                                if timer_utils.has_value() {
                                    timer_utils.start();
                                    timer_ctx.set_timer_mode(TimerMode::RunningMode);
                                    timer_btn.change_state(BtnMode::Stop);
                                } else {
                                    timer_ctx.set_timer_mode(TimerMode::NormalMode);
                                    timer_btn.change_state(BtnMode::Set);
                                }
                            }
                            BtnMode::Reset => {
                                timer_utils.reset();
                                timer_ctx.set_timer_mode(TimerMode::NormalMode);
                                timer_btn.change_state(BtnMode::Set);
                            }
                            BtnMode::Stop => {
                                timer_utils.stop();
                                timer_ctx.set_timer_mode(TimerMode::NormalMode);
                                timer_btn.change_state(BtnMode::Start);
                            }
                            BtnMode::Set => {
                                timer_ctx.set_timer_mode(TimerMode::SetterMode);
                                timer_btn.change_state(BtnMode::Set);
                            }
                        }
                    } else if KeyCode::Enter == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() == &TimerMode::SetterMode
                    {
                        timer_ctx.set_timer_mode(TimerMode::NormalMode);
                        if timer_utils.has_value() {
                            timer_btn.change_state(BtnMode::Start);
                        } else {
                            timer_btn.change_state(BtnMode::Set);
                        }
                    } else if KeyCode::Tab == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() == &TimerMode::SetterMode
                    {
                        match timer_ctx.get_timer_sagment() {
                            TimerSagemnt::Hour => timer_ctx.set_timer_sagment(TimerSagemnt::Sec),
                            TimerSagemnt::Min => timer_ctx.set_timer_sagment(TimerSagemnt::Hour),
                            TimerSagemnt::Sec => timer_ctx.set_timer_sagment(TimerSagemnt::Min),
                        }
                    } else if KeyCode::Up == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() == &TimerMode::SetterMode
                    {
                        match timer_ctx.get_timer_sagment() {
                            TimerSagemnt::Hour => timer_utils.increase_hour(),
                            TimerSagemnt::Min => timer_utils.increase_min(),
                            TimerSagemnt::Sec => timer_utils.increase_sec(),
                        }
                    } else if KeyCode::Down == key.code
                        && self.mode == TickTermMode::Timer
                        && timer_ctx.get_timer_mode() == &TimerMode::SetterMode
                    {
                        match timer_ctx.get_timer_sagment() {
                            TimerSagemnt::Hour => timer_utils.decrease_hour(),
                            TimerSagemnt::Min => timer_utils.decrease_min(),
                            TimerSagemnt::Sec => timer_utils.decrease_sec(),
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
        timer_ctx: Option<&TimerCtx>,
        timer_utils: Option<&TimerUtils>,
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
                    if self.mode == TickTermMode::Normal {
                        "<Esc>Quit | <w>Stopwatch | <t>Timer | <i>Toggle-KeyBinds"
                    } else if self.mode == TickTermMode::StopWatch {
                        "<Esc>Quit | <b>Back | <Tab>Switch-Button | <Enter>Click | <i>Toggle-KeyBinds"
                    } else if self.mode == TickTermMode::Timer
                        && timer_ctx.is_some()
                        && timer_ctx.unwrap().get_timer_mode() == &TimerMode::SetterMode
                    {
                        "<Esc>Quit | <b>Back | <Enter>Exit-Setter | <Tab>Switch-Value | <Up/Down>Change-Value | <i>Toggle-KeyBinds"
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
        if self.mode == TickTermMode::Normal {
            if let Some(t) = t {
                TimeUI(frame, t);
            }
        } else if self.mode == TickTermMode::StopWatch
            && let Some(w) = w
            && let Some(btn) = btn
        {
            StopWatchUI(frame, w, btn);
        } else if self.mode == TickTermMode::Timer
            && let Some(timer_ctx) = timer_ctx
            && let Some(btn) = btn
            && let Some(timer_utils) = timer_utils
        {
            TimerUI(frame, timer_ctx, btn, timer_utils);
        }
    }
}
