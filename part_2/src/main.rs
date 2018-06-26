extern crate tcod;

mod entity;

use tcod::console::{Root, Console};
use tcod::FontLayout;
use tcod::FontType;
use tcod::colors;
use tcod::BackgroundFlag;
use tcod::input;
use tcod::input::Key;
use tcod::input::KeyCode;
use entity::Entity;


enum Action {
    MovePlayer(i32, i32),
    Fullscreen,
    Exit,
}

fn handle_keys(key: Option<Key>) -> Option<Action> {
    match key {
        Some(Key { code: KeyCode::Left, .. }) => Some(Action::MovePlayer(-1, 0)),
        Some(Key { code: KeyCode::Right, .. }) => Some(Action::MovePlayer(1, 0)),
        Some(Key { code: KeyCode::Up, .. }) => Some(Action::MovePlayer(0, -1)),
        Some(Key { code: KeyCode::Down, .. }) => Some(Action::MovePlayer(0, 1)),
        Some(Key { code: KeyCode::Escape, .. }) => Some(Action::Exit),
        Some(Key { code: KeyCode::Enter, alt: true, .. }) => Some(Action::Fullscreen),
        _ => None
    }
}

fn main() {
    let screen_width = 80;
    let screen_height = 50;

    let mut entities = vec![
        Entity::new(screen_width / 2, screen_height / 2, '@', colors::WHITE),
        Entity::new(screen_width / 2 - 5, screen_height / 2, '@', colors::YELLOW),
    ];

    let player_entity_index: usize = 0;

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part1")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();
    root.set_default_foreground(colors::WHITE);

    while !root.window_closed() {
        root.clear();
        root.flush();

        let action = handle_keys(root.check_for_keypress(tcod::input::KEY_PRESSED));
        match action {
            Some(Action::Exit) => break,
            Some(Action::Fullscreen) => {
                let is_fullscreen = root.is_fullscreen();
                root.set_fullscreen(!is_fullscreen)
            }
            Some(Action::MovePlayer(move_x, move_y)) => entities[player_entity_index].mv((move_x, move_y)),
            _ => ()
        }
    }
}

