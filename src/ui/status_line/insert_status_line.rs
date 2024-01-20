use crate::app::App;

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
    widgets::Paragraph,
    Frame,
};

pub fn draw_insert_mode_status_line(f: &mut Frame, app: &App, chunk: Rect) {
    // Mode indicator
    let input_mode_style = Style::default()
        .bg(app.theme.red)
        .fg(app.theme.crust)
        .bold();
    let mode_padding_front = Span::styled(" ", input_mode_style);
    let mode = Span::styled(app.mode.to_string().to_uppercase(), input_mode_style);
    let mode_padding_back = Span::styled(" ", input_mode_style);
    let mode_indicator = vec![mode_padding_front, mode, mode_padding_back];

    // Content
    let path_padding_front = Span::styled(" ", Style::default().bg(app.theme.mantle));
    let path = if let Some(path) = &app.editor.path {
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                Span::styled(filename_str, Style::default().bg(app.theme.mantle))
            } else {
                Span::styled(
                    "Cannot get filepath",
                    Style::default().fg(app.theme.red).bg(app.theme.mantle),
                )
            }
        } else {
            Span::styled(
                "Cannot get filepath",
                Style::default().fg(app.theme.red).bg(app.theme.mantle),
            )
        }
    } else {
        Span::styled("New File", Style::default().bg(app.theme.mantle))
    };
    let content = vec![path_padding_front, path];

    // Combine the spans
    let status_line_spans = [mode_indicator, content].concat();
    let status_line_widget = Paragraph::new(ratatui::text::Line::from(status_line_spans))
        .style(Style::default().bg(app.theme.mantle));
    f.render_widget(status_line_widget, chunk);
}
