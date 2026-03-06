use crate::app::{App, signal::signals_to_char};
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn ui(f: &mut Frame, app: &App) {
    f.render_widget(
        Block::bordered()
            .title_top(" Telegraph ")
            .title_bottom(" Space: send morse | c: Change case | L: Clear output "),
        Rect::new(0, 0, f.area().width, f.area().height),
    );

    let buf = Paragraph::new(app.buf.clone()).wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(
        buf,
        Rect::new(2, f.area().height - 6, f.area().width - 4, f.area().height),
    );

    render_input_pane(f, app);

    if app.show_debug {
        render_debug_pane(f, app);
    }
}

fn render_input_pane(f: &mut Frame, app: &App) {
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

        Line::from(vec!["> ".into(), prev, staged])
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

        let p = format!("{s} => {}", app.apply_case(c.unwrap_or('?')));

        f.render_widget(
            Span::from(p).gray(),
            Rect::new(3, top_y + 2, f.area().width, f.area().height),
        );
    }
}

fn render_debug_pane(f: &mut Frame, app: &App) {
    let width = 19;
    let height = 6;

    let top_x = f.area().width - width - 2;
    let top_y = f.area().height - height - 1;

    // Render the block, clearing all elements underneath.
    // https://ratatui.rs/recipes/render/overwrite-regions/
    {
        let block_rect = Rect::new(top_x, top_y, width, height);
        ratatui::widgets::Clear.render(block_rect, f.buffer_mut());
        f.render_widget(
            Block::bordered()
                .title_top(" Debug ")
                .border_type(ratatui::widgets::BorderType::Double)
                .yellow(),
            block_rect,
        );
    }

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
}
