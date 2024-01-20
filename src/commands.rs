use crate::app::{editor::motions::Motion, InputMode};

#[derive(Debug, Clone)]
pub enum Command {
    Quit,
    CommandLineStop,
    CommandLineInsertChar(char),
    CommandLineDelete,
    CommandLineLeft,
    CommandLineRight,
    CommandLineEnter,
    ChangeInputMode(InputMode),
    EditorInsert(String),
    EditorDelete(Motion),
    EditorMove(Motion),
    EditorSave,
}
