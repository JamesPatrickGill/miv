pub mod command_line;
pub mod editor;
pub mod theme;

use std::error;

use strum_macros::{Display, EnumString};

use crate::commands::Command;

use self::{command_line::CommandLine, editor::EditorBuffer, theme::Theme};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// The editing mode of the application
#[derive(Display, Debug, EnumString, PartialEq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Editing mode of application
    pub mode: InputMode,
    /// Data for maintaining command input
    pub command_line: CommandLine,
    /// Colors for display
    pub theme: Theme,
    /// The buffer being edited
    pub editor: EditorBuffer,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            mode: InputMode::Normal,
            command_line: CommandLine::default(),
            theme: Theme::default(),
            editor: EditorBuffer::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(file: String) -> Self {
        Self {
            running: true,
            mode: InputMode::Normal,
            command_line: CommandLine::default(),
            theme: Theme::default(),
            editor: EditorBuffer::from_file(file),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Execute collection of commands
    pub fn execute(&mut self, commands: Vec<Command>) -> AppResult<()> {
        for command in commands {
            self.execute_single_command(command)?;
        }
        Ok(())
    }

    /// An internal function to run a single command
    fn execute_single_command(&mut self, command: Command) -> AppResult<()> {
        match command {
            Command::Quit => self.quit(),
            Command::CommandLineStop => self.command_line.deactivate(),
            Command::CommandLineInsertChar(ch) => self.command_line.enter_char(ch),
            Command::CommandLineDelete => self.command_line.delete_char(),
            Command::CommandLineLeft => self.command_line.move_cursor_left(),
            Command::CommandLineRight => self.command_line.move_cursor_right(),
            Command::CommandLineEnter => {
                self.execute(self.command_line.get_commands())?;
            }
            Command::ChangeInputMode(mode) => self.change_input_mode(mode),
            Command::EditorInsert(to_insert) => self.editor.insert(to_insert, self.mode),
            Command::EditorDelete(motion) => self.editor.delete(motion, self.mode),
            Command::EditorMove(motion) => self.editor.move_cursor(&motion, self.mode),
            Command::EditorSave => self.editor.save()?,
        };
        Ok(())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Change input mode
    fn change_input_mode(&mut self, input_mode: InputMode) {
        self.mode = input_mode
    }
}
