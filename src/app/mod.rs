use self::cli::generate_matches;
use self::signal::{Signal, signals_to_char};

pub mod cli;
mod config;
mod signal;

#[derive(Default)]
pub struct App {
    pub pressed_begin: Option<std::time::Instant>,
    pub latest_event: Option<std::time::Instant>,

    pub staged_signal: Option<Signal>,
    pub signals: Vec<Signal>,

    pub render_case: RenderCase,

    pub buf: String,

    /// Indicates if the user has begun quitting the app.
    is_closing: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum RenderCase {
    #[default]
    Lowercase,
    Uppercase,
}

impl App {
    fn update_latest(&mut self) {
        self.latest_event = Some(std::time::Instant::now());
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub const fn cycle_render_case(&mut self) {
        self.render_case = match self.render_case {
            RenderCase::Lowercase => RenderCase::Uppercase,
            RenderCase::Uppercase => RenderCase::Lowercase,
        }
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

    fn update_signal(&mut self) {
        let Some(init) = self.pressed_begin else {
            self.pressed_begin = Some(std::time::Instant::now());
            self.staged_signal = Some(Signal::Dot);
            return;
        };

        if init.elapsed() > std::time::Duration::from_millis(101) {
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
