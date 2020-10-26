use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use super::actions;
use super::viewport::Viewport;

// https://github.com/crossterm-rs/crossterm/blob/master/src/event.rs
pub fn get_user_input() -> Option<(KeyCode, KeyModifiers)> {
    enable_raw_mode().expect("Failed to enable raw mode to get user input");
    let key = read().expect("Could not get user input");
    disable_raw_mode().expect("Failed to disable raw mode after getting user input");
    if let Event::Key(KeyEvent { code, modifiers }) = key {
        return Some((code, modifiers));
    }
    None
}

pub fn process_user_input(viewport: &mut Viewport) -> Option<actions::PlayerAction> {
    let user_input = get_user_input();
    if let Some((inpt, modifiers)) = user_input {
        let from_input_result = actions::from_input(inpt, modifiers);
        match from_input_result {
            Some(actions::PlayerAction::MoveUp) => viewport.move_up(1),
            Some(actions::PlayerAction::MoveUp10) => viewport.move_up(10),
            Some(actions::PlayerAction::MoveDown) => viewport.move_down(1),
            Some(actions::PlayerAction::MoveDown10) => viewport.move_down(10),
            Some(actions::PlayerAction::MoveLeft) => viewport.move_left(1),
            Some(actions::PlayerAction::MoveLeft10) => viewport.move_left(10),
            Some(actions::PlayerAction::MoveRight) => viewport.move_right(1),
            Some(actions::PlayerAction::MoveRight10) => viewport.move_right(10),
            Some(actions::PlayerAction::ZoomToggle) => {
                if viewport.zoom > 1 {
                    viewport.zoom = 1;
                } else {
                    viewport.zoom = 10;
                }
            }
            _ => {}
        }
        return from_input_result;
    }
    None
}
