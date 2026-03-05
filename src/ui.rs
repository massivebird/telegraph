use crate::app::App;
use ratatui::{Frame, layout::Rect, text::Text, widgets::Block};

pub fn ui(f: &mut Frame, app: &App) {
    let block = Block::bordered().title_top(" Telegraph ").title_bottom(" Space: send morse | c: Change case | l: Clear output ");

    f.render_widget(block, Rect::new(0, 0, f.area().width, f.area().height));

    let text = Text::from(if app.is_pressed() { "PRESSED" } else { "" });
    f.render_widget(text, Rect::new(1, 1, f.area().width, f.area().height));

    let latest = Text::from(
        app.latest_event
            .map(|i| format!("latest event: {}", i.elapsed().as_millis().to_string()))
            .unwrap_or(String::new()),
    );
    f.render_widget(latest, Rect::new(15, 1, f.area().width, f.area().height));

    let latest = Text::from(
        app.pressed_begin
            .map(|i| format!("pressed elapsed: {}", i.elapsed().as_millis().to_string()))
            .unwrap_or(String::new()),
    );
    f.render_widget(latest, Rect::new(15, 2, f.area().width, f.area().height));

    let symbol = Text::from(app.staged_signal.map(|s| s.to_string()).unwrap_or_default());
    f.render_widget(symbol, Rect::new(1, 2, f.area().width, f.area().height));

    let signals = Text::from(format!("{:?}", app.signals));
    f.render_widget(signals, Rect::new(1, 3, f.area().width, f.area().height));

    let buf = Text::from(match app.render_case {
        crate::app::RenderCase::Lowercase => app.buf.clone().to_lowercase(),
        crate::app::RenderCase::Uppercase => app.buf.clone().to_uppercase(),
    });
    f.render_widget(buf, Rect::new(1, 4, f.area().width, f.area().height));
}
