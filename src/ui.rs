use crate::app::App;
use ratatui::{Frame, layout::Rect, text::Text, widgets::Block};

pub fn ui(f: &mut Frame, app: &App) {
    let block = Block::bordered().title_top(" Morse ");

    f.render_widget(block, Rect::new(0, 0, f.area().width, f.area().height));

    let text = Text::from(if app.is_pressed() { "PRESSED" } else { "" });
    f.render_widget(text, Rect::new(1, 1, f.area().width, f.area().height));

    let latest = Text::from(
        app.pressed_begin
            .map(|i| i.elapsed().as_millis().to_string())
            .unwrap_or(String::new()),
    );
    f.render_widget(latest, Rect::new(15, 2, f.area().width, f.area().height));

    let symbol = Text::from(app.staged_signal.map(|s| s.to_string()).unwrap_or_default());
    f.render_widget(symbol, Rect::new(1, 2, f.area().width, f.area().height));

    let signals = Text::from(format!("{:?}", app.signals));
    f.render_widget(signals, Rect::new(1, 3, f.area().width, f.area().height));

    let buf = Text::from(app.buf.clone());
    f.render_widget(buf, Rect::new(1, 4, f.area().width, f.area().height));
}
