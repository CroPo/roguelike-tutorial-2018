extern crate tcod;
extern crate rand;
extern crate textwrap;

mod ecs;
mod render;
mod map_objects;
mod game_states;
mod message;

use tcod::console::{Root, Offscreen};
use tcod::FontLayout;
use tcod::FontType;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::FovAlgorithm;

use ecs::Entity;
use map_objects::map::GameMap;
use map_objects::fov;
use game_states::GameStates;

use ecs::Ecs;
use ecs::component::Position;
use ecs::component::MonsterAi;
use ecs::action::EntityAction;
use ecs::component::Actor;
use ecs::component::Corpse;
use message::MessageLog;
use render::MessagePanel;
use std::rc::Rc;

enum Action {
    MovePlayer(i32, i32),
    Fullscreen,
    Exit,
}

fn handle_keys(key: Option<Key>) -> Option<Action> {
    match key {
        Some(Key { code: KeyCode::Left, .. }) | Some(Key { printable: 'h', .. }) => Some(Action::MovePlayer(-1, 0)),
        Some(Key { code: KeyCode::Right, .. }) | Some(Key { printable: 'l', .. }) => Some(Action::MovePlayer(1, 0)),
        Some(Key { code: KeyCode::Up, .. }) | Some(Key { printable: 'k', .. }) => Some(Action::MovePlayer(0, -1)),
        Some(Key { code: KeyCode::Down, .. }) | Some(Key { printable: 'j', .. }) => Some(Action::MovePlayer(0, 1)),
        Some(Key { printable: 'y', .. }) => Some(Action::MovePlayer(-1, -1)),
        Some(Key { printable: 'u', .. }) => Some(Action::MovePlayer(1, -1)),
        Some(Key { printable: 'b', .. }) => Some(Action::MovePlayer(-1, 1)),
        Some(Key { printable: 'n', .. }) => Some(Action::MovePlayer(1, 1)),
        Some(Key { code: KeyCode::Escape, .. }) => Some(Action::Exit),
        Some(Key { code: KeyCode::Enter, alt: true, .. }) => Some(Action::Fullscreen),
        _ => None
    }
}

fn main() {
    let screen_width = 80;
    let screen_height = 50;

    let bar_width = 20;
    let panel_height = 7;

    let message_x = bar_width + 2;

    let panel_y = screen_height - panel_height;
    let message_width = screen_width - bar_width - 2;
    let message_height = panel_height - 1;

    let map_width = 80;
    let map_height = 43;

    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;

    let fov_algorithm = FovAlgorithm::Basic;
    let fov_light_walls = true;
    let fov_radius = 10;

    let mut fov_recompute = true;

    let max_monsters_per_room = 3;

    let mut log = Rc::new(MessageLog::new());
    let mut ecs = Ecs::initialize();

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part 7: Creating the Interface")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    let mut con = Offscreen::new(screen_width, screen_height);
    let mut panel = Offscreen::new(screen_width, panel_height);

    let mut map = GameMap::new(map_width, map_height);
    map.make_map(max_rooms, room_min_size, room_max_size, &mut ecs, max_monsters_per_room);

    let mut fov_map = fov::initialize_fov(&map);

    let mut game_state = GameStates::PlayersTurn;

    let log_panel = MessagePanel::new((message_x, 0), (message_width, message_height), Rc::clone(&log));

    while !root.window_closed() {
        if fov_recompute {
            let player_pos = ecs.get_component::<Position>(ecs.player_entity_id).unwrap();
            fov::recompute_fov(&mut fov_map, (player_pos.position.0, player_pos.position.1), fov_radius, fov_light_walls, fov_algorithm);
        }

        ::render::render_all(&ecs, &mut map, &fov_map, fov_recompute,
                             &mut con, &mut panel, &mut root,
                             bar_width, panel_height, panel_y, &log_panel);
        root.flush();
        ::render::clear_all(&ecs, &mut con);

        fov_recompute = true;

        let action = handle_keys(root.check_for_keypress(tcod::input::KEY_PRESSED));
        match action {
            Some(Action::Exit) => break,
            Some(Action::Fullscreen) => {
                let is_fullscreen = root.is_fullscreen();
                root.set_fullscreen(!is_fullscreen)
            }
            Some(Action::MovePlayer(vel_x, vel_y)) => if game_state == GameStates::PlayersTurn {
                let id = ecs.player_entity_id;

                let destination = {
                    let p = ecs.get_component::<Position>(id).unwrap();
                    (p.position.0 + vel_x, p.position.1 + vel_y)
                };

                let action = if !map.is_move_blocked(destination.0, destination.1) {
                    let targets = Position::is_blocked_by(&ecs, destination);

                    if let Some(target_id) = targets.iter().next() {
                        let player_creature = ecs.get_component::<Actor>(id).unwrap();
                        match player_creature.calculate_attack(&ecs, *target_id) {
                            Some(x) => EntityAction::TakeDamage(*target_id, x, id),
                            None => EntityAction::Idle
                        }
                    } else {
                        EntityAction::MoveRelative(id, (vel_x, vel_y))
                    }
                } else {
                    EntityAction::Idle
                };

                action.execute(&mut ecs, Rc::clone(&log));

                game_state = GameStates::EnemyTurn;
            }
            _ => if game_state == GameStates::EnemyTurn {
                let entity_ids = ecs.get_all_ids::<MonsterAi>();

                entity_ids.iter().for_each(|entity_id| {
                    let action = match ecs.get_component::<MonsterAi>(*entity_id) {
                        Some(ai) => ai.calculate_turn(&ecs, &map),
                        _ => EntityAction::Idle
                    };
                    action.execute(&mut ecs, Rc::clone(&log))
                });

                game_state = if ecs.has_component::<Corpse>(ecs.player_entity_id) {
                    GameStates::PlayerDead
                } else {
                    GameStates::PlayersTurn
                };
            }
        }
    }
}

