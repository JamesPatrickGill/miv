pub mod command_mode;
pub mod insert_mode;
pub mod normal_mode;

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::InputMode, commands::Command};

use self::{command_mode::COMMAND_MAP, insert_mode::INSERT_MAP, normal_mode::NORMAL_MAP};

#[derive(Debug, Clone)]
pub enum Keymap {
    One(Vec<Command>),
    Many(HashMap<String, Keymap>),
}

#[derive(Debug)]
pub struct InputStack {
    event_stack: Vec<String>,
    potentials: Option<HashMap<String, Keymap>>,
}

impl Default for InputStack {
    fn default() -> Self {
        Self::new()
    }
}

impl InputStack {
    pub fn new() -> Self {
        Self {
            event_stack: vec![],
            potentials: None,
        }
    }

    fn character_input(&self, event: KeyEvent, mode: &InputMode) -> Option<char> {
        if self.potentials.is_some() && !self.event_stack.is_empty() {
            return None;
        }

        if let InputMode::Normal = mode {
            return None;
        }

        match event {
            KeyEvent {
                code: KeyCode::Char(_),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => None,
            KeyEvent {
                code: KeyCode::Char(ch),
                ..
            } => Some(ch),
            _ => None,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent, mode: &InputMode) -> Option<Vec<Command>> {
        // First we will shortcircuit if we need to handle an editor InputMode
        if let Some(ch) = self.character_input(event, mode) {
            match mode {
                InputMode::Normal => {}
                InputMode::Insert => {
                    return Some(vec![Command::EditorInsert(ch.into())]);
                }
                InputMode::Command => {
                    return Some(vec![Command::CommandLineInsertChar(ch)]);
                }
            }
        }

        let key = get_valid_key_string_from_event(event);

        let candidate = if let Some(valid_key_press) = key {
            self.event_stack.push(valid_key_press.clone());

            let pivot: &HashMap<String, Keymap> = if let Some(pot) = &self.potentials {
                pot
            } else {
                match mode {
                    InputMode::Normal => &NORMAL_MAP,
                    InputMode::Insert => &INSERT_MAP,
                    InputMode::Command => &COMMAND_MAP,
                }
            };

            pivot.get(&valid_key_press).cloned()
        } else {
            None
        };

        match candidate {
            Some(km) => match km {
                Keymap::Many(many) => {
                    self.potentials = Some(many);
                    None
                }
                Keymap::One(one) => {
                    self.potentials = None;
                    self.event_stack.clear();
                    Some(one)
                }
            },
            None => {
                self.potentials = None;
                self.event_stack.clear();
                None
            }
        }
    }
}

fn get_valid_key_string_from_event(event: KeyEvent) -> Option<String> {
    match event.code {
        KeyCode::Backspace => Some("back".into()),
        KeyCode::Enter => Some("enter".into()),
        KeyCode::Left => Some("left".into()),
        KeyCode::Right => Some("right".into()),
        KeyCode::Up => Some("up".into()),
        KeyCode::Down => Some("down".into()),
        KeyCode::Tab => Some("tab".into()),
        KeyCode::BackTab => Some("backtab".into()),
        KeyCode::Char(ch) => Some(ch.into()),
        KeyCode::Esc => Some("esc".into()),
        _ => None,
    }
    .map(|key_as_str| match event.modifiers {
        KeyModifiers::CONTROL => format!("ctrl+{}", key_as_str),
        _ => key_as_str,
    })
}
