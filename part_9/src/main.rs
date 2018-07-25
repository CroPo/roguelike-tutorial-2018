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

use map_objects::map::GameMap;
use map_objects::fov;
use game_states::GameState;

use ecs::Ecs;
use ecs::component::Position;
use ecs::component::MonsterAi;
use ecs::action::EntityAction;
use ecs::component::Actor;
use ecs::component::Corpse;
use message::MessageLog;
use render::MessagePanel;
use std::rc::Rc;
use tcod::input::Event;
use tcod::input::Mouse;
use ecs::id::EntityId;
use ecs::component::Item;
use render::render_all;
use render::clear_all;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    Exit,
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

    let mut mouse_pos = (0, 0);

    let max_monsters_per_room = 3;
    let max_items_per_room = 2;

    let log = Rc::new(MessageLog::new());
    let mut ecs = Ecs::initialize();

    let mut root = Root::initializer()
        .size(screen_width, screen_height)
        .title("/r/roguelikedev Tutorial Part 8: Items and Inverntory")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    let mut con = Offscreen::new(screen_width, screen_height);
    let mut panel = Offscreen::new(screen_width, panel_height);

    let mut map = GameMap::new(map_width, map_height);
    map.make_map(max_rooms, room_min_size, room_max_size, &mut ecs, max_monsters_per_room, max_items_per_room);

    let mut fov_map = fov::initialize_fov(&map);

    let mut game_state = GameState::PlayersTurn;

    let log_panel = MessagePanel::new((message_x, 0), (message_width, message_height), Rc::clone(&log));

    'game_loop: while !root.window_closed() {

        {
            let player_pos = ecs.get_component::<Position>(ecs.player_entity_id).unwrap();
            fov::recompute_fov(&mut fov_map, (player_pos.position.0, player_pos.position.1), fov_radius, fov_light_walls, fov_algorithm);
        }

        render_all(&ecs, &mut map, &fov_map, &game_state,
                   &mut con, &mut panel, &mut root,
                   bar_width, panel_y, &log_panel, mouse_pos);
        clear_all(&ecs, &mut con);


        let result = game_state.run(&mut ecs, Rc::clone(&log), &map);

        if let Some(engine_action) = result.engine_action {
            match engine_action {
                EngineAction::Exit => break 'game_loop,
                EngineAction::ToggleFullscreen => {
                    let fullscreen = root.is_fullscreen();
                    root.set_fullscreen(!fullscreen)
                },
                EngineAction::MousePos(x, y) => {
                    mouse_pos.0 = x as i32;
                    mouse_pos.1 = y as i32;
                }
            }
        }

        game_state = result.next_state;
    }
}

