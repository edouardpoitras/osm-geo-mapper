use crossterm::event::{KeyCode, KeyModifiers};

#[derive(PartialEq)]
pub enum PlayerAction {
    MoveUp,
    MoveUp10,
    MoveDown,
    MoveDown10,
    MoveLeft,
    MoveLeft10,
    MoveRight,
    MoveRight10,
    ZoomToggle,
    LoadMoreData,
    Quit,
}

pub fn from_input(code: KeyCode, modifiers: KeyModifiers) -> Option<PlayerAction> {
    let shift = modifiers == KeyModifiers::SHIFT;
    // User movement.
    // Handle individual characters.
    if let KeyCode::Char('k') = code {
        if shift {
            return Some(PlayerAction::MoveUp10);
        }
        return Some(PlayerAction::MoveUp);
    } else if let KeyCode::Char('j') = code {
        if shift {
            return Some(PlayerAction::MoveDown10);
        }
        return Some(PlayerAction::MoveDown);
    } else if let KeyCode::Char('h') = code {
        if shift {
            return Some(PlayerAction::MoveLeft10);
        }
        return Some(PlayerAction::MoveLeft);
    } else if let KeyCode::Char('l') = code {
        if shift {
            return Some(PlayerAction::MoveRight10);
        }
        return Some(PlayerAction::MoveRight);
    } else if let KeyCode::Char('z') = code {
        return Some(PlayerAction::ZoomToggle);
    }
    // Handle arrow keys.
    if let KeyCode::Up = code {
        if shift {
            return Some(PlayerAction::MoveUp10);
        }
        return Some(PlayerAction::MoveUp);
    } else if let KeyCode::Down = code {
        if shift {
            return Some(PlayerAction::MoveDown10);
        }
        return Some(PlayerAction::MoveDown);
    } else if let KeyCode::Left = code {
        if shift {
            return Some(PlayerAction::MoveLeft10);
        }
        return Some(PlayerAction::MoveLeft);
    } else if let KeyCode::Right = code {
        if shift {
            return Some(PlayerAction::MoveRight10);
        }
        return Some(PlayerAction::MoveRight);
    }
    // Handle request to load more data.
    if let KeyCode::Enter = code {
        return Some(PlayerAction::LoadMoreData);
    }
    // Handle quitting.
    if let KeyCode::Esc = code {
        return Some(PlayerAction::Quit);
    } else if let KeyCode::Char('q') = code {
        return Some(PlayerAction::Quit);
    }
    None
}
