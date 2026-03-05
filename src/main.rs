use self::{app::App, ui::ui};
use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
mod ui;

fn main() -> eyre::Result<()> {
    let mut app = App::generate()?;

    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it.
    let ui_refresh_rate = Duration::from_millis(100);
    let res = start_app(&mut terminal, ui_refresh_rate, &mut app);

    // App is quitting!
    // Restore terminal and environment to normal.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn start_app<B: Backend>(
    terminal: &mut Terminal<B>,
    ui_refresh_rate: Duration,
    app: &mut App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        let timeout = ui_refresh_rate.saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)? {
            handle_events(app)?;
        } else if app.pressed_begin.is_some() {
            app.press();
        } else {
            app.try_push_char();
        }

        if app.is_closing() {
            return Ok(());
        }

        if last_tick.elapsed() >= ui_refresh_rate {
            last_tick = Instant::now();
        }
    }
}

/// Handles user input.
fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char(' ') && key.kind == KeyEventKind::Release {
            app.release();
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q' | 'Q') => app.close(),
            KeyCode::Char(' ') => app.press(),
            _ => (),
        }
    }

    Ok(())
}
