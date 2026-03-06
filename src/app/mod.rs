use std::time::Duration;

use self::cli::generate_matches;
use self::signal::{Signal, signals_to_char};

pub mod cli;
mod config;
pub mod signal;

#[derive(Default)]
pub struct App {
    pub pressed_begin: Option<std::time::Instant>,
    pub latest_event: Option<std::time::Instant>,

    pub staged_signal: Option<Signal>,
    pub signals: Vec<Signal>,

    render_case: RenderCase,

    pub buf: String,

    pub latest_char: Option<Vec<Signal>>,

    pub show_debug: bool,

    /// Indicates if the user has begun quitting the app.
    is_closing: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum RenderCase {
    #[default]
    Uppercase,
    Lowercase,
}

impl App {
    fn update_latest(&mut self) {
        self.latest_event = Some(std::time::Instant::now());
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.latest_char = None;
    }

    pub fn cycle_render_case(&mut self) {
        self.render_case = match self.render_case {
            RenderCase::Lowercase => {
                self.buf = self.buf.to_uppercase();
                RenderCase::Uppercase
            }
            RenderCase::Uppercase => {
                self.buf = self.buf.to_lowercase();
                RenderCase::Lowercase
            }
        }
    }

    pub const fn apply_case(&self, c: char) -> char {
        match self.render_case {
            RenderCase::Lowercase => c.to_ascii_lowercase(),
            RenderCase::Uppercase => c.to_ascii_uppercase(),
        }
    }

    pub fn try_push_char(&mut self) {
        if self.pressed_begin.is_some()
            || self.signals.is_empty()
            || self
                .latest_event
                .is_none_or(|i| i.elapsed() < Duration::from_millis(500))
        {
            return;
        }

        self.latest_event = None;

        let signals = std::mem::take(&mut self.signals);

        // Process some prosigns.
        // https://en.wikipedia.org/wiki/Prosigns_for_Morse_code
        {
            use Signal::{Dash, Dot};
            match &signals[..] {
                [Dot, Dot, Dot, Dot, Dot, Dot, Dot, Dot] => {
                    let _ = self.buf.pop();
                    return;
                }
                [Dot, Dot, Dot, Dot, Dot, Dot, Dot, Dot, Dot, Dash, Dot, Dash] => {
                    self.clear();
                    return;
                }
                _ => (),
            }
        }

        self.latest_char = Some(signals.clone());

        let Some(c) = signals_to_char(&signals).map(|c| self.apply_case(c)) else {
            return;
        };

        self.buf.push(c);
    }

    pub const fn is_pressed(&self) -> bool {
        self.pressed_begin.is_some()
    }

    pub fn release(&mut self) {
        self.update_latest();

        self.signals.push(self.staged_signal.unwrap());

        self.pressed_begin = None;
        self.staged_signal = None;
    }

    pub fn press(&mut self) {
        self.update_latest();
        self.update_signal();
    }

    fn update_signal(&mut self) {
        let Some(init) = self.pressed_begin else {
            self.pressed_begin = Some(std::time::Instant::now());
            self.staged_signal = Some(Signal::Dot);
            return;
        };

        if init.elapsed() > Duration::from_millis(180) {
            self.staged_signal = Some(Signal::Dash);
        } else {
            self.staged_signal = Some(Signal::Dot);
        }
    }

    pub fn generate() -> eyre::Result<Self> {
        let matches: clap::ArgMatches = generate_matches();

        Ok(Self {
            ..Default::default()
        })
    }

    pub const fn close(&mut self) {
        self.is_closing = true;
    }

    pub const fn is_closing(&self) -> bool {
        self.is_closing
    }
}
