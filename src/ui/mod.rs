// mod editors;
mod editor;
mod status_line;

use ratatui::prelude::*;

use crate::app::App;

use self::{editor::draw_editor, status_line::draw_status_line};

pub fn render(f: &mut Frame, app: &App) {
    let global_layout_constraints = vec![Constraint::Min(1), Constraint::Length(1)];
    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(global_layout_constraints)
        .split(f.size());

    draw_editor(f, app, global_layout[0]);
    draw_status_line(f, app, global_layout[1])
}
