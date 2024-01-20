use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{app::InputMode, commands::Command};

use super::Keymap;

lazy_static! {
    pub static ref COMMAND_MAP: HashMap<String, Keymap> = {
        HashMap::from([
            (
                "esc".into(),
                Keymap::One(vec![
                    Command::CommandLineStop,
                    Command::ChangeInputMode(InputMode::Normal),
                ]),
            ),
            (
                "enter".into(),
                Keymap::One(vec![
                    Command::CommandLineEnter,
                    Command::CommandLineStop,
                    Command::ChangeInputMode(InputMode::Normal),
                ]),
            ),
            ("back".into(), Keymap::One(vec![Command::CommandLineDelete])),
            ("left".into(), Keymap::One(vec![Command::CommandLineLeft])),
            ("right".into(), Keymap::One(vec![Command::CommandLineRight])),
        ])
    };
}
