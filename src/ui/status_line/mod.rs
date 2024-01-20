mod command_status_line;
mod insert_status_line;
mod normal_status_line;

use ratatui::{prelude::Rect, Frame};

use crate::app::{App, InputMode};

use self::{
    command_status_line::draw_command_mode_status_line,
    insert_status_line::draw_insert_mode_status_line,
    normal_status_line::draw_normal_mode_status_line,
};

pub fn draw_status_line(f: &mut Frame, app: &App, chunk: Rect) {
    match app.mode {
        InputMode::Normal => draw_normal_mode_status_line(f, app, chunk),
        InputMode::Insert => draw_insert_mode_status_line(f, app, chunk),
        InputMode::Command => draw_command_mode_status_line(f, app, chunk),
    };
}
