extern crate tcod;
extern crate rand;

mod ecs;
mod render;
mod map_objects;
mod game_states;

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
use ecs::creature::CreatureTemplate;
use ecs::component::Position;
use ecs::component::MonsterAi;
use ecs::action::EntityAction;

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

    let mut ecs = Ecs::initialize();

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part 6: Doing (and taking) some damage")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    let mut con = Offscreen::new(screen_width, screen_height);

    let mut map = GameMap::new(map_width, map_height);
    map.make_map(max_rooms, room_min_size, room_max_size, &mut ecs, max_monsters_per_room);

    let mut fov_map = fov::initialize_fov(&map);

    let mut game_state = GameStates::PlayersTurn;

    while !root.window_closed() {
        if fov_recompute {
            let player_pos = ecs.get_component::<Position>(ecs.player_entity_id).unwrap();
            fov::recompute_fov(&mut fov_map, (player_pos.position.0, player_pos.position.1), fov_radius, fov_light_walls, fov_algorithm);
        }

        ::render::render_all(&ecs, &mut map, &fov_map, fov_recompute, &mut con, &mut root);
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


                let mut destination = {
                    let player_pos = ecs.get_component::<Position>(id).unwrap();
                    (player_pos.position.0 + vel_x, player_pos.position.1 + vel_y)
                };

                if !map.is_move_blocked(destination.0, destination.1) {
                    let bump_into =
                        {
                            let targets = Position::is_blocked_by(&ecs, destination);

                            for e in &targets {
                                println!("You kick a Monster in the shins, much to its annoyance!")
                            }

                            !targets.is_empty()
                        };

                    if !bump_into {
                        let player_pos = ecs.get_component_mut::<Position>(id).unwrap();
                        player_pos.mv((vel_x, vel_y))
                    }
                }

                game_state = GameStates::EnemyTurn;
            }
            _ => if game_state == GameStates::EnemyTurn {

                let entity_ids = ecs.get_all_ids::<MonsterAi>();

                entity_ids.iter().for_each(|entity_id| {
                    let action = match ecs.get_component::<MonsterAi>(*entity_id) {
                        Some(ai) => ai.calculate_turn(&ecs, &map),
                        _ => EntityAction::Idle
                    };
                    action.execute(&mut ecs)
                });

                game_state = GameStates::PlayersTurn;
            }
        }
    }
}

