extern crate tcod;
extern crate rand;

mod entity;
mod render;
mod map_objects;

use tcod::console::{Root, Console};
use tcod::FontLayout;
use tcod::FontType;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode;
use entity::Entity;
use map_objects::map::GameMap;

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
    let map_width = 80;
    let map_height = 45;

    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;

    let fov_algorithm = 0;
    let fov_light_walls = true;
    let fov_radius = 10;

    let mut entities = vec![
        Entity::new(screen_width / 2, screen_height / 2, '@', colors::WHITE),
        Entity::new(screen_width / 2 - 5, screen_height / 2, '@', colors::YELLOW),
    ];

    let player_entity_index: usize = 0;

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part4")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();
    root.set_default_foreground(colors::WHITE);

    let mut map = GameMap::new(map_width, map_height);
    map.make_map(max_rooms, room_min_size, room_max_size, &mut entities[player_entity_index]);

    while !root.window_closed() {

        ::render::render_all(&entities, &map,&mut root, screen_width, screen_height);
        root.flush();
        ::render::clear_all(&entities, &mut root);

        let action = handle_keys(root.check_for_keypress(tcod::input::KEY_PRESSED));
        match action {
            Some(Action::Exit) => break,
            Some(Action::Fullscreen) => {
                let is_fullscreen = root.is_fullscreen();
                root.set_fullscreen(!is_fullscreen)
            }
            Some(Action::MovePlayer(move_x, move_y)) => {
                let mut player = &mut entities[player_entity_index];
                if !map.is_move_blocked(player.pos.0 + move_x, player.pos.1 + move_y) {
                    player.mv((move_x, move_y))
                }
            }
            _ => ()
        }
    }
}

