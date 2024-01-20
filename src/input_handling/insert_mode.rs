use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    app::{editor::motions::Motion, InputMode},
    commands::Command,
};

use super::Keymap;

lazy_static! {
    pub static ref INSERT_MAP: HashMap<String, Keymap> = {
        HashMap::from([
            (
                "esc".into(),
                Keymap::One(vec![
                    Command::ChangeInputMode(InputMode::Normal),
                    // The following is a fun hack that isn't too hard
                    // to actually fix.
                    // Have fun finding what it's a hack for.
                    Command::EditorMove(Motion::CharBackward),
                    Command::EditorMove(Motion::CharForward ),
                ]),
            ),
            (
                "back".into(),
                Keymap::One(vec![Command::EditorDelete(Motion::CharBackward)]),
            ),
            (
                "right".into(),
                Keymap::One(vec![Command::EditorMove(Motion::CharForward)]),
            ),
            (
                "left".into(),
                Keymap::One(vec![Command::EditorMove(Motion::CharBackward)]),
            ),
            (
                "enter".into(),
                Keymap::One(vec![Command::EditorInsert("\n".into())]),
            ),
        ])
    };
}
