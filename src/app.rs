use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use std::time::Duration;

use crate::ui;

pub const TARGET_TEXT: &str = "The quick brown fox jumps over the lazy dog.";
const TICK_RATE: Duration = Duration::from_millis(16);

pub struct App {
    pub text: &'static str,
    pub states: Vec<Option<bool>>,
    pub cursor: usize,
    pub error_count: usize,
    pub finished: bool,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let text = TARGET_TEXT;
        Self {
            text,
            states: vec![None; text.len()],
            cursor: 0,
            error_count: 0,
            finished: false,
            should_quit: false,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| ui::render(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(TICK_RATE)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c')
                            if key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL) =>
                        {
                            self.should_quit = true;
                        }
                        KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        KeyCode::Backspace => {
                            if self.cursor > 0 {
                                self.cursor -= 1;
                                self.states[self.cursor] = None;
                            }
                        }
                        KeyCode::Char(ch) => {
                            if self.cursor < self.text.len() {
                                let expected =
                                    self.text.as_bytes()[self.cursor] as char;
                                if ch == expected {
                                    self.states[self.cursor] = Some(true);
                                } else {
                                    self.states[self.cursor] = Some(false);
                                    self.error_count += 1;
                                }
                                self.cursor += 1;
                                if self.cursor == self.text.len() {
                                    self.finished = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
