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
use tcod::map::FovAlgorithm;

use entity::Entity;
use map_objects::map::GameMap;
use map_objects::fov;

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

    let fov_algorithm = FovAlgorithm::Basic;
    let fov_light_walls = true;
    let fov_radius = 10;

    let mut fov_recompute = true;

    let max_monsters_per_room = 3;

    let mut entities = vec![
        Entity::new(0,0, '@', colors::WHITE),
    ];

    let player_entity_index: usize = 0;

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part5")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();
    root.set_default_foreground(colors::WHITE);

    let mut map = GameMap::new(map_width, map_height);

    map.make_map(max_rooms, room_min_size, room_max_size, &mut entities, player_entity_index, max_monsters_per_room,);

    let mut fov_map = fov::initialize_fov(&map);

    while !root.window_closed() {

        if fov_recompute {
            let player = &entities[player_entity_index];
            fov::recompute_fov(&mut fov_map, (player.pos.0, player.pos.1), fov_radius, fov_light_walls, fov_algorithm);
        }

        ::render::render_all(&entities, &mut map, &fov_map, fov_recompute, &mut root);
        root.flush();
        ::render::clear_all(&entities, &mut root);

        fov_recompute = false;

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
                    fov_recompute = true;
                    player.mv((move_x, move_y))
                }
            }
            _ => ()
        }
    }
}

