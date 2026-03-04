use self::cli::generate_matches;
use std::fmt::Display;

pub mod cli;
mod config;

#[derive(Default)]
pub struct App {
    pub pressed_begin: Option<std::time::Instant>,
    pub latest_event: Option<std::time::Instant>,

    pub staged_signal: Option<Signal>,
    pub signals: Vec<Signal>,

    pub buf: String,

    /// Indicates if the user has begun quitting the app.
    is_closing: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum Signal {
    Dot,
    Dash,
}

pub fn signals_to_char(signals: &[Signal]) -> Option<char> {
    use Signal::{Dash, Dot};

    match signals {
        [Dot, Dash] => Some('a'),
        [Dash, Dot, Dot, Dot] => Some('b'),
        [Dash, Dot, Dash, Dot] => Some('c'),
        [Dash, Dot, Dot] => Some('d'),
        [Dot] => Some('e'),
        [Dot, Dot, Dash, Dot] => Some('f'),
        [Dash, Dash, Dot] => Some('g'),
        [Dot, Dot, Dot, Dot] => Some('h'),
        [Dot, Dot] => Some('i'),
        [Dot, Dash, Dash, Dash] => Some('j'),
        [Dash, Dot, Dash] => Some('k'),
        [Dot, Dash, Dot, Dot] => Some('l'),
        [Dash, Dash] => Some('m'),
        [Dash, Dot] => Some('n'),
        [Dash, Dash, Dash] => Some('o'),
        [Dot, Dash, Dash, Dot] => Some('p'),
        [Dash, Dash, Dot, Dash] => Some('q'),
        [Dot, Dash, Dot] => Some('r'),
        [Dot, Dot, Dot] => Some('s'),
        [Dash] => Some('t'),
        [Dot, Dot, Dash] => Some('u'),
        [Dot, Dot, Dot, Dash] => Some('v'),
        [Dot, Dash, Dash] => Some('w'),
        [Dash, Dot, Dot, Dash] => Some('x'),
        [Dash, Dot, Dash, Dash] => Some('y'),
        [Dash, Dash, Dot, Dot] => Some('z'),
        _ => None,
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Dot => ".",
            Self::Dash => "-",
        };

        write!(f, "{c}")
    }
}

impl App {
    fn update_latest(&mut self) {
        self.latest_event = Some(std::time::Instant::now());
    }

    pub fn try_push_char(&mut self) {
        if self.pressed_begin.is_some()
            || self.signals.is_empty()
            || self
                .latest_event
                .is_none_or(|i| i.elapsed() < std::time::Duration::from_millis(500))
        {
            return;
        }

        let signals = std::mem::take(&mut self.signals);
        let c = signals_to_char(&signals).unwrap_or('?');

        self.buf.push(c);

        self.latest_event = None;
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

    pub fn update_signal(&mut self) {
        if let Some(init) = self.pressed_begin
            && init.elapsed() > std::time::Duration::from_millis(100)
        {
            self.staged_signal = Some(Signal::Dash);
        } else {
            self.pressed_begin = Some(std::time::Instant::now());
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
