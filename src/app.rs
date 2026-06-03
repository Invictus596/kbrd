use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use std::time::Duration;

use crate::ui;

const PLACEHOLDER_TEXT: &str = "The quick brown fox jumps over the lazy dog.";
const TICK_RATE: Duration = Duration::from_millis(16);

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self, terminal: &mut Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| ui::render(frame, PLACEHOLDER_TEXT))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(TICK_RATE)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            self.should_quit = true;
                        }
                        KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
