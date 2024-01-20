use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{editor::highlighting::HighlightGroup, App};

pub fn draw_editor(f: &mut Frame, app: &App, chunk: Rect) {
    let colors = &app.theme;
    let content = app.editor.gap_buffer.get_text_as_string();
    let highlight_spans = &app.editor.highlight_groups;

    let mut lines_as_spans = vec![];
    let mut current_line = vec![];

    let mut cursor_coord = (0, 0);
    let mut cursor_index_render_accumulator = 0;
    let mut cursor_coord_found = false;

    for span in highlight_spans {
        let slice_to_highlight = &content[span.start..span.end];
        let style = match span.group {
            HighlightGroup::Attribute => Style::default().fg(colors.rose),
            HighlightGroup::Comment => Style::default().fg(colors.surface1),
            HighlightGroup::Constant => Style::default().fg(colors.peach),
            HighlightGroup::ConstantBuiltin => Style::default().fg(colors.peach),
            HighlightGroup::Constructor => Style::default().fg(colors.peach),
            HighlightGroup::Function => Style::default().fg(colors.blue),
            HighlightGroup::FunctionBuiltin => Style::default().fg(colors.blue),
            HighlightGroup::FunctionMethod => Style::default().fg(colors.blue),
            HighlightGroup::FunctionMacro => Style::default().fg(colors.blue),
            HighlightGroup::Keyword => Style::default().fg(colors.mauve),
            HighlightGroup::Property => Style::default().fg(colors.lavender),
            HighlightGroup::Punctuation => Style::default().fg(colors.green),
            HighlightGroup::PunctuationDelimiter => Style::default().fg(colors.text),
            HighlightGroup::String => Style::default().fg(colors.green),
            HighlightGroup::StringSpecial => Style::default().fg(colors.green),
            HighlightGroup::Type => Style::default().fg(colors.yellow),
            HighlightGroup::TypeBuiltin => Style::default().fg(colors.sapphire),
            HighlightGroup::Variable => Style::default().fg(colors.text),
            HighlightGroup::VariableBuiltin => Style::default().fg(colors.text),
            HighlightGroup::VariableParameter => Style::default().fg(colors.yellow),
            HighlightGroup::None => Style::default().fg(colors.text),
            _ => Style::default().fg(colors.text),
        };

        for line in slice_to_highlight.split_inclusive('\n') {
            if line.is_empty() || line.ends_with('\n') {
                current_line.push(Span::styled(line, style));
                lines_as_spans.push(current_line.clone());
                current_line.clear();
            } else {
                current_line.push(Span::styled(line, style));
            }
        }
    }
    if !current_line.is_empty() {
        lines_as_spans.push(current_line);
    }

    if content.ends_with('\n') {
        lines_as_spans.push(vec![Span::raw("")]);
    }

    let max_line_number_digits = (lines_as_spans.len()).to_string().len();

    let mut lines = vec![];
    for mut line in lines_as_spans {
        let width = line.iter().fold(0, |acc, span| acc + span.width());
        if !cursor_coord_found {
            if width + 1 + cursor_index_render_accumulator > app.editor.cursor_index {
                let x = app.editor.cursor_index - cursor_index_render_accumulator;
                let y = lines.len();
                cursor_coord_found = true;
                cursor_coord = (x, y);
            } else {
                cursor_index_render_accumulator += width + 1;
            }
        }
        let line_number = lines.len() + 1;
        let leading_spacing = max_line_number_digits - (line_number).to_string().len();
        let mut prefix = vec![
            Span::styled(
                " ".repeat(leading_spacing),
                Style::default().fg(colors.yellow),
            ),
            Span::styled(
                (line_number).to_string(),
                Style::default().fg(colors.yellow),
            ),
            Span::styled(
                " ".repeat(max_line_number_digits + 2),
                Style::default().fg(colors.yellow),
            ),
        ];

        prefix.append(&mut line);
        lines.push(Line::from(prefix));
    }

    let text_content = Paragraph::new(lines).style(Style::default().bg(colors.crust));
    f.render_widget(text_content, chunk);
    f.set_cursor(
        chunk.x + cursor_coord.0 as u16 + ((2 * max_line_number_digits) as u16 + 2),
        chunk.y + cursor_coord.1 as u16,
    )
}
