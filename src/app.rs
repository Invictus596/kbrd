use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::ui;

const TICK_RATE: Duration = Duration::from_millis(16);

const SENTENCES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "Pack my box with five dozen liquor jugs.",
    "How vexingly quick daft zebras jump.",
    "Sphinx of black quartz judge my vow.",
    "The five boxing wizards jump quickly.",
    "Jackdaws love my big sphinx of quartz.",
    "Cozy lummox gives smart squid who asks for job pen.",
];

#[derive(Default, Clone)]
pub struct KeyStats {
    pub hits: usize,
    pub errors: usize,
    pub total_time: Duration,
}

impl KeyStats {
    pub fn total(&self) -> usize {
        self.hits + self.errors
    }
    pub fn accuracy(&self) -> f64 {
        let t = self.total();
        if t == 0 {
            return 1.0;
        }
        self.hits as f64 / t as f64
    }
    pub fn avg_time(&self) -> Duration {
        let t = self.total();
        if t == 0 {
            return Duration::ZERO;
        }
        self.total_time / t as u32
    }
}

pub struct App {
    pub text: String,
    pub states: Vec<Option<bool>>,
    pub cursor: usize,
    pub error_count: usize,
    pub finished: bool,
    pub show_results: bool,
    pub key_stats: HashMap<char, KeyStats>,
    char_start: Vec<Instant>,
    pub session_start: Instant,
    pub session_end: Option<Instant>,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let text = pick_sentence();
        let now = Instant::now();
        Self {
            states: vec![None; text.len()],
            char_start: vec![now; text.len()],
            text,
            cursor: 0,
            error_count: 0,
            finished: false,
            show_results: false,
            key_stats: HashMap::new(),
            session_start: now,
            session_end: None,
            should_quit: false,
        }
    }

    pub fn reset(&mut self) {
        let text = pick_sentence();
        let now = Instant::now();
        self.text = text;
        self.states = vec![None; self.text.len()];
        self.char_start = vec![now; self.text.len()];
        self.cursor = 0;
        self.error_count = 0;
        self.finished = false;
        self.show_results = false;
        self.key_stats.clear();
        self.session_start = now;
        self.session_end = None;
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
                            if self.show_results {
                                self.should_quit = true;
                            } else if self.finished {
                                self.show_results = true;
                            } else {
                                self.should_quit = true;
                            }
                        }
                        KeyCode::Backspace => {
                            if self.show_results {
                                return Ok(());
                            }
                            if self.cursor > 0 {
                                self.cursor -= 1;
                                self.states[self.cursor] = None;
                            }
                        }
                        KeyCode::Enter => {
                            if self.show_results {
                                self.reset();
                            }
                        }
                        KeyCode::Char(ch) => {
                            if self.show_results {
                                return Ok(());
                            }
                            if self.cursor < self.text.len() {
                                let elapsed = self.char_start[self.cursor].elapsed();
                                let expected =
                                    self.text.as_bytes()[self.cursor] as char;

                                if ch == expected {
                                    self.states[self.cursor] = Some(true);
                                } else {
                                    self.states[self.cursor] = Some(false);
                                    self.error_count += 1;
                                }

                                let entry = self
                                    .key_stats
                                    .entry(expected)
                                    .or_default();
                                if ch == expected {
                                    entry.hits += 1;
                                } else {
                                    entry.errors += 1;
                                }
                                entry.total_time += elapsed;

                                self.cursor += 1;
                                if self.cursor < self.text.len() {
                                    self.char_start[self.cursor] = Instant::now();
                                }

                                if self.cursor == self.text.len() {
                                    self.finished = true;
                                    self.show_results = true;
                                    self.session_end = Some(Instant::now());
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

fn pick_sentence() -> String {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let idx = (seed % SENTENCES.len() as u128) as usize;
    SENTENCES[idx].to_string()
}
