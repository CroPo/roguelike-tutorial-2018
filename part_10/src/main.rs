extern crate tcod;
extern crate rand;
extern crate textwrap;

mod ecs;
mod render;
mod map_objects;
mod game_states;
mod message;
mod settings;

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
use settings::Settings;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    Exit,
}

fn main() {

    let mut mouse_pos = (0, 0);
    let settings = Settings::new();

    let log = Rc::new(MessageLog::new());
    let mut ecs = Ecs::initialize();

    let mut root = Root::initializer()
        .size(settings.screen_width(), settings.screen_height())
        .title(settings.title())
        .font(settings.font_path(), settings.font_layout())
        .font_type(settings.font_type())
        .init();

    let mut map = GameMap::new(settings.map_width(), settings.map_height());

    map.make_map(&mut ecs, &settings);

    let mut fov_map = fov::initialize_fov(&map);

    let mut game_state = GameState::PlayersTurn;

    let log_panel = MessagePanel::new(settings.message_pos(),
                                      settings.message_dimensions(),
                                      Rc::clone(&log));

    'game_loop: while !root.window_closed() {
        {
            let player_pos = ecs.get_component::<Position>(ecs.player_entity_id).unwrap();
            fov::recompute_fov(&mut fov_map, (player_pos.position.0, player_pos.position.1), &settings);
        }

        render_all(&ecs, &mut root, &settings, &mut map, &fov_map, &game_state, &log_panel, mouse_pos);

        let result = game_state.run(&mut ecs, &fov_map, Rc::clone(&log), &map);

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

