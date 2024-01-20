pub mod filetypes;
pub mod gap_buffer;
pub mod highlighting;
pub mod motions;

use std::fmt::{self, Debug};
use std::fs::write;
use std::{env, fs, path::PathBuf};

use tracing::info;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

use self::highlighting::HighlightSpan;
use self::motions::Motion;
use self::{
    filetypes::FileType,
    gap_buffer::GapBuffer,
    highlighting::{get_highlighting_config, get_highlighting_function, HighlightingFn},
};

use super::{AppResult, InputMode};

pub struct EditorBuffer {
    /// Position of cursor in the editor area.
    pub cursor_index: usize,
    /// Current cursor line
    pub cursor_line: usize,
    /// Current cursor column
    pub cursor_col: usize,
    /// Gap buffer storing text
    pub gap_buffer: GapBuffer,
    /// Gap buffer storing text
    pub path: Option<PathBuf>,
    pub filetype: FileType,
    /// Highlighting utilities
    pub highlighter: Highlighter,
    pub highlighter_config: HighlightConfiguration,
    pub highlight_groups: Vec<HighlightSpan>,
    highlighting_function: HighlightingFn,
}

impl Debug for EditorBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EditorBuffer")
            .field("cursor_index", &self.cursor_index)
            .field("gap_buffer", &self.gap_buffer)
            .field("path", &self.path)
            .field("filetype", &self.filetype)
            .finish()
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        let highlighter = Highlighter::new();
        let highlighter_config = get_highlighting_config(FileType::Rust);
        let highlighter_function = get_highlighting_function(FileType::Rust);

        EditorBuffer {
            cursor_index: 0,
            cursor_line: 0,
            cursor_col: 0,
            gap_buffer: GapBuffer::with_data(""),
            path: None,
            filetype: FileType::Rust,
            highlighter,
            highlighter_config,
            highlight_groups: vec![],
            highlighting_function: highlighter_function,
        }
    }
}

impl EditorBuffer {
    pub fn from_file(file: String) -> Self {
        let cwd_path = env::current_dir().unwrap();
        let full_path = cwd_path.join(file);
        let source =
            fs::read_to_string(&full_path).expect("Should have been able to read the file");

        let mut eb = EditorBuffer {
            gap_buffer: GapBuffer::with_data(&source),
            path: Some(full_path),
            ..Self::default()
        };
        eb.calculate_highlights();
        eb
    }

    pub fn save(&self) -> AppResult<()> {
        if let Some(path) = &self.path {
            write(path, self.gap_buffer.get_text_as_string())?;
            return Ok(());
        }
        Err("Failed to save".into())
    }

    pub fn insert(&mut self, to_insert: String, mode: InputMode) {
        self.gap_buffer.insert_at(&to_insert, self.cursor_index);

        self.move_cursor(&Motion::CharForward, mode);
        self.calculate_highlights();
    }

    pub fn move_cursor(&mut self, motion: &Motion, mode: InputMode) {
        match motion {
            Motion::CharForward => self.move_forward_char(),
            Motion::CharBackward => self.move_backward_char(),
            Motion::NextWordStart => self.move_forward_to_word_start(),
            Motion::NextWordProperStart => self.move_forward_to_word_proper_start(),
            Motion::NextWordEnd => self.move_forward_to_word_end(),
            Motion::NextWordProperEnd => self.move_forward_to_word_proper_end(),
            Motion::LastWordStart => self.move_backward_to_word_start(),
            Motion::LastWordProperStart => self.move_backward_to_word_proper_start(),
            Motion::LastWordEnd => self.move_backward_to_word_end(),
            Motion::LastWordProperEnd => self.move_backward_to_word_proper_end(),
            Motion::LineStart => self.move_to_line_start(),
            Motion::LineEnd => self.move_to_line_end(),
            Motion::NextLineStart => self.move_to_next_line_start(),
        }

        let final_char = self.gap_buffer.get_at(self.cursor_index);
        if let InputMode::Normal = mode {
            if final_char == '\n' {
                self.cursor_index -= 1
            }
        };
    }

    pub fn delete(&mut self, motion: Motion, mode: InputMode) {
        let delete_start = self.cursor_index;
        self.move_cursor(&motion, mode);
        let candidate_char = self.gap_buffer.get_at(self.cursor_index);
        info!("{:?}", candidate_char);
        let delete_end = self.cursor_index;
        let (amount_to_delete, at) = if delete_end < delete_start {
            (delete_start - delete_end, delete_end)
        } else {
            (delete_end - delete_start, delete_start)
        };

        self.gap_buffer.delete_at(amount_to_delete, at);
        self.cursor_index = at;
        self.calculate_highlights();
    }

    pub fn calculate_highlights(&mut self) {
        let content = self.gap_buffer.get_text_as_bytes();
        self.highlight_groups =
            (self.highlighting_function)(&content, &mut self.highlighter, &self.highlighter_config);
    }

    fn move_forward_char(&mut self) {
        let candidate_index = self.cursor_index + 1;

        let data_length = self.gap_buffer.data_length();

        if candidate_index >= data_length {
            self.cursor_index = data_length
        } else {
            self.cursor_index = candidate_index
        };
    }

    fn move_backward_char(&mut self) {
        let candidate_index = self.cursor_index.saturating_sub(1);

        self.cursor_index = candidate_index;
    }

    fn move_forward_to_word_start(&mut self) {
        let start_char = self.gap_buffer.get_at(self.cursor_index);
        let start_is_alphanumeric = start_char.is_alphanumeric();

        let mut whitspace_seen = start_char.is_whitespace();
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            if !candidate_char.is_whitespace() {
                if whitspace_seen {
                    self.cursor_index = candidate_index;
                    return;
                };

                if start_is_alphanumeric && !candidate_char.is_alphanumeric() {
                    self.cursor_index = candidate_index;
                    return;
                };

                if !start_is_alphanumeric && candidate_char.is_alphanumeric() {
                    self.cursor_index = candidate_index;
                    return;
                };
            } else {
                whitspace_seen = true;
            }

            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }

    fn move_forward_to_word_proper_start(&mut self) {
        let mut whitspace_seen = self.gap_buffer.get_at(self.cursor_index).is_whitespace();
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length {
            let candidate_char = self.gap_buffer.get_at(candidate_index);

            if whitspace_seen && !candidate_char.is_whitespace() {
                self.cursor_index = candidate_index;
                return;
            };

            if candidate_char.is_whitespace() {
                whitspace_seen = true;
            };

            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }

    fn move_forward_to_word_end(&mut self) {
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length - 1 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            let candidate_char_is_alphanumeric = candidate_char.is_alphanumeric();
            let next_index = candidate_index + 1;
            let next_char = self.gap_buffer.get_at(next_index);

            if !candidate_char.is_whitespace()
                && ((candidate_char_is_alphanumeric ^ next_char.is_alphanumeric())
                    || next_char.is_whitespace())
            {
                self.cursor_index = candidate_index;
                return;
            }
            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }

    fn move_forward_to_word_proper_end(&mut self) {
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length - 1 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            let next_index = candidate_index + 1;
            let next_char = self.gap_buffer.get_at(next_index);

            if next_char.is_whitespace() && !candidate_char.is_whitespace() {
                self.cursor_index = candidate_index;
                return;
            };

            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }

    fn move_backward_to_word_start(&mut self) {
        let mut candidate_index = self.cursor_index.saturating_sub(1);
        while candidate_index > 0 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            let candidate_char_is_alphanumeric = candidate_char.is_alphanumeric();
            let next_index = candidate_index - 1;
            let next_char = self.gap_buffer.get_at(next_index);

            if !candidate_char.is_whitespace()
                && ((candidate_char_is_alphanumeric ^ next_char.is_alphanumeric())
                    || next_char.is_whitespace())
            {
                self.cursor_index = candidate_index;
                return;
            }
            candidate_index -= 1
        }

        self.cursor_index = 0;
    }

    fn move_backward_to_word_proper_start(&mut self) {
        let mut candidate_index = self.cursor_index.saturating_sub(1);

        while candidate_index > 0 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            let next_index = candidate_index - 1;
            let next_char = self.gap_buffer.get_at(next_index);

            if next_char.is_whitespace() && !candidate_char.is_whitespace() {
                self.cursor_index = candidate_index;
                return;
            };

            candidate_index -= 1
        }

        self.cursor_index = 0;
    }

    fn move_backward_to_word_end(&mut self) {
        let start_char = self.gap_buffer.get_at(self.cursor_index);
        let start_is_alphanumeric = start_char.is_alphanumeric();

        let mut whitspace_seen = start_char.is_whitespace();
        let mut candidate_index = self.cursor_index - 1;

        while candidate_index > 0 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            if !candidate_char.is_whitespace() {
                if whitspace_seen {
                    self.cursor_index = candidate_index;
                    return;
                };

                if start_is_alphanumeric && !candidate_char.is_alphanumeric() {
                    self.cursor_index = candidate_index;
                    return;
                };

                if !start_is_alphanumeric && candidate_char.is_alphanumeric() {
                    self.cursor_index = candidate_index;
                    return;
                };
            } else {
                whitspace_seen = true;
            }

            candidate_index -= 1
        }

        self.cursor_index = 0;
    }

    fn move_backward_to_word_proper_end(&mut self) {
        let mut whitspace_seen = self.gap_buffer.get_at(self.cursor_index).is_whitespace();
        let mut candidate_index = self.cursor_index - 1;

        while candidate_index > 0 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);

            if whitspace_seen && !candidate_char.is_whitespace() {
                self.cursor_index = candidate_index;
                return;
            };

            if candidate_char.is_whitespace() {
                whitspace_seen = true;
            };

            candidate_index -= 1
        }

        self.cursor_index = 0;
    }

    fn move_to_line_start(&mut self) {
        let mut candidate_index = self.cursor_index.saturating_sub(1);
        while candidate_index > 0 {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            if candidate_char == '\n' {
                self.cursor_index = candidate_index + 1;
                return;
            }
            candidate_index -= 1
        }

        self.cursor_index = 0;
    }

    fn move_to_next_line_start(&mut self) {
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            if candidate_char == '\n' {
                self.cursor_index = candidate_index + 1;
                return;
            }
            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }

    fn move_to_line_end(&mut self) {
        let mut candidate_index = self.cursor_index + 1;
        let data_length = self.gap_buffer.data_length();

        while candidate_index < data_length {
            let candidate_char = self.gap_buffer.get_at(candidate_index);
            if candidate_char == '\n' {
                self.cursor_index = candidate_index;
                return;
            }
            candidate_index += 1
        }

        self.cursor_index = data_length - 1;
    }
}
