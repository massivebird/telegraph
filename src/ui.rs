use crate::app::App;
use crate::app::signal::signals_to_char;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::{Frame, layout::Rect, text::Text, widgets::Block};

pub fn ui(f: &mut Frame, app: &App) {
    let block = Block::bordered()
        .title_top(" Telegraph ")
        .title_bottom(" Space: send morse | c: Change case | l: Clear output ");

    f.render_widget(block, Rect::new(0, 0, f.area().width, f.area().height));

    let buf = Text::from(match app.render_case {
        crate::app::RenderCase::Lowercase => app.buf.clone().to_lowercase(),
        crate::app::RenderCase::Uppercase => app.buf.clone().to_uppercase(),
    });
    f.render_widget(buf, Rect::new(2, f.area().height - 6, f.area().width - 4, f.area().height));

    render_input(f, app);

    if app.show_debug {
        render_debug(f, app);
    }
}

fn render_input(f: &mut Frame, app: &App) {
    let height = 3;

    let top_y = f.area().height - height - 1;

    f.render_widget(
        Block::bordered()
            .title_top(" Input ")
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact),
        Rect::new(0, top_y, f.area().width, f.area().height),
    );

    let signals: Line<'_> = {
        let prev = Span::from(
            app.signals
                .iter()
                .fold(String::new(), |acc, x| acc + &x.to_string()),
        );

        let staged = app
            .staged_signal
            .map(|s| s.to_string())
            .unwrap_or_default()
            .blue();

        Line::from(vec![prev, staged])
    };

    f.render_widget(
        signals,
        Rect::new(1, top_y + 1, f.area().width, f.area().height),
    );

    if let Some(signals) = &app.latest_char {
        let s = signals
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string());

        let c = signals_to_char(signals);

        let p = format!("{s} => {}", c.unwrap_or('?'));

        f.render_widget(
            Span::from(p).gray(),
            Rect::new(1, top_y + 2, f.area().width, f.area().height),
        );
    }
}

fn render_debug(f: &mut Frame, app: &App) {
    let width = 19;
    let height = 6;

    let top_x = f.area().width - width - 2;
    let top_y = f.area().height - height - 1;

    let block = Block::bordered().title_top(" Debug ").yellow();

    f.render_widget(block, Rect::new(top_x, top_y, width, height));

    let text = Text::from(if app.is_pressed() { "PRESSED" } else { "" });
    f.render_widget(
        text,
        Rect::new(top_x + 1, top_y + 1, f.area().width, f.area().height),
    );

    let latest = Text::from(app.latest_event.map_or(String::new(), |i| {
        format!("latest: {}", i.elapsed().as_millis())
    }));
    f.render_widget(
        latest,
        Rect::new(top_x + 1, top_y + 3, f.area().width, f.area().height),
    );

    let latest = Text::from(app.pressed_begin.map_or(String::new(), |i| {
        format!("pressed: {}", i.elapsed().as_millis())
    }));
    f.render_widget(
        latest,
        Rect::new(top_x + 1, top_y + 4, f.area().width, f.area().height),
    );

    let symbol = Text::from(app.staged_signal.map(|s| s.to_string()).unwrap_or_default());
    f.render_widget(
        symbol,
        Rect::new(top_x + 1, top_y + 2, f.area().width, f.area().height),
    );

    let buf = Text::from(match app.render_case {
        crate::app::RenderCase::Lowercase => app.buf.clone().to_lowercase(),
        crate::app::RenderCase::Uppercase => app.buf.clone().to_uppercase(),
    });
    f.render_widget(buf, Rect::new(1, 4, f.area().width, f.area().height));
}
