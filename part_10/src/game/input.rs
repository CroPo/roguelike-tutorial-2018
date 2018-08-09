use tcod::input::KEY_PRESS;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::input::Mouse;
use tcod::input::Event;
use tcod::input::EventFlags;
use game::state::GameState;

/// Action are triggered by the input (mouse & keys)
pub enum InputAction {
    MovePlayer(i32, i32),
    MousePos(isize, isize),
    SelectEntity(isize, isize),
    SelectOption(char),
    PickUp,
    ShowInventory,
    ShowInventoryDrop,
    Fullscreen,
    Exit,
}


pub fn handle_input(state: &GameState, event: Option<(EventFlags, Event)>) -> Option<InputAction> {
    if let Some(e) = event {
        match e {
            (KEY_PRESS, Event::Key(key)) => {
                match state {
                    GameState::PlayersTurn => handle_keys_player_turn(key),
                    GameState::ShowInventoryUse | GameState::ShowInventoryDrop => handle_keys_selection_menu(key),
                    _ => handle_keys_default(key),
                }
            }
            (_, Event::Mouse(mouse)) => match state {
                GameState::Targeting( .. ) => handle_mouse_targeting(mouse),
                _ => handle_mouse_default(mouse)
            },
            _ => None
        }
    } else {
        None
    }
}

fn handle_mouse_targeting(mouse: Mouse) -> Option<InputAction> {
    match mouse {
        Mouse { lbutton_pressed: true, .. } => Some(InputAction::SelectEntity(mouse.cx, mouse.cy)),
        Mouse { .. } => Some(InputAction::MousePos(mouse.cx, mouse.cy)),
    }
}

fn handle_mouse_default(mouse: Mouse) -> Option<InputAction> {
    match mouse {
        Mouse { .. } => Some(InputAction::MousePos(mouse.cx, mouse.cy)),
    }
}

fn handle_keys_player_turn(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Left, .. } | Key { printable: 'h', .. } => Some(InputAction::MovePlayer(-1, 0)),
        Key { code: KeyCode::Right, .. } | Key { printable: 'l', .. } => Some(InputAction::MovePlayer(1, 0)),
        Key { code: KeyCode::Up, .. } | Key { printable: 'k', .. } => Some(InputAction::MovePlayer(0, -1)),
        Key { code: KeyCode::Down, .. } | Key { printable: 'j', .. } => Some(InputAction::MovePlayer(0, 1)),
        Key { printable: 'y', .. } => Some(InputAction::MovePlayer(-1, -1)),
        Key { printable: 'u', .. } => Some(InputAction::MovePlayer(1, -1)),
        Key { printable: 'b', .. } => Some(InputAction::MovePlayer(-1, 1)),
        Key { printable: 'n', .. } => Some(InputAction::MovePlayer(1, 1)),
        Key { printable: 'i', .. } => Some(InputAction::ShowInventory),
        Key { printable: 'd', .. } => Some(InputAction::ShowInventoryDrop),
        Key { printable: 'g', .. } => Some(InputAction::PickUp),
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        Key { code: KeyCode::Enter, alt: true, .. } => Some(InputAction::Fullscreen),
        _ => None
    }
}

fn handle_keys_selection_menu(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        Key { printable, .. } => Some(InputAction::SelectOption(printable)),
    }
}

fn handle_keys_default(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        _ => None
    }
}