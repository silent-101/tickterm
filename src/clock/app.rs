#![allow(non_snake_case)]

use std::{sync::mpsc, thread, time::Duration};

use crate::clock::{ui, utils::Time};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct Clock {
    is_running: bool,
}

impl Clock {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn runner(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.is_running = true;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx.send(key).is_err() {
                        break; // Main thread exited, stop listening
                    }
                }
            }
        });

        let mut current_time = Time::default();

        while self.is_running {
            current_time.set_new_time();
            terminal.draw(|frame| {
                self.dom(frame, &current_time);
            })?;
            match rx.recv_timeout(Duration::from_millis(999)) {
                Ok(key) => {
                    if let KeyCode::Esc = key.code {
                        self.is_running = false;
                    }
                }
                Err(_) => {
                    // Timeout - continue to next frame
                }
            }
        }
        Ok(())
    }

    fn dom(&mut self, frame: &mut Frame, t: &Time) {
        ui::TimeUI(frame, t)
    }
}
