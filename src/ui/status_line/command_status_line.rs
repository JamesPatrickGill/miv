use crate::app::App;

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw_command_mode_status_line(f: &mut Frame, app: &App, chunk: Rect) {
    // Mode indicator
    let input_mode_style = Style::default()
        .bg(app.theme.sapphire)
        .fg(app.theme.crust)
        .bold();
    let mode_padding_front = Span::styled(" ", input_mode_style);
    let mode = Span::styled(app.mode.to_string().to_uppercase(), input_mode_style);
    let mode_padding_back = Span::styled(" ", input_mode_style);
    let mode_indicator = vec![mode_padding_front, mode, mode_padding_back];

    // Content
    let input_padding_front = Span::styled(" ⟫ ", Style::default().fg(app.theme.rose).bold());
    let command_line_input = Span::styled(app.command_line.value.as_str(), Style::default());
    let content = vec![input_padding_front, command_line_input];

    // Combine the spans
    let status_line_spans = [mode_indicator, content].concat();
    let status_line_widget =
        Paragraph::new(Line::from(status_line_spans)).style(Style::default().bg(app.theme.mantle));
    f.render_widget(status_line_widget, chunk);
    f.set_cursor(
        chunk.x + 12 + app.command_line.cursor_position as u16,
        chunk.y,
    );
}
