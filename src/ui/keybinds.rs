use std::collections::HashMap;

use crossterm::event::KeyCode;

pub enum Action {
    Quit,
    Call(fn())
}

pub fn default_keybinds() -> HashMap<KeyCode, Action> {
    let mut map: HashMap<KeyCode, Action> = HashMap::new();
    
    map.insert(KeyCode::Char(' '), Action::Call(super::pause));
    map.insert(KeyCode::Char('q'), Action::Quit);

    return map
}