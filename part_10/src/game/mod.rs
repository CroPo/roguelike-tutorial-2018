use std::rc::Rc;

use tcod::console::Root;
use tcod::Map;

use json::JsonValue;

use ecs::Ecs;
use ecs::component::Position;

use map_objects::map::GameMap;
use message::MessageLog;
use settings::Settings;
use game::state::GameState;
use render::MessagePanel;
use map_objects::fov;
use render::render_all;
use savegame;
use savegame::{Serialize, Deserialize};

pub mod state;
pub mod input;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    Exit(bool),
}

pub struct Game <'a> {
    ecs: Ecs,
    map: GameMap,
    log: Rc<MessageLog>,

    settings: &'a Settings,
    state: GameState,
    mouse_pos: (i32, i32),
    fov_map: Map,
    log_panel: MessagePanel
}

impl <'a> Game <'a>  {
    pub fn new(settings: &Settings) -> Game {

        let mut ecs = Ecs::initialize();
        let mut map = GameMap::new(settings.map_width(), settings.map_height());
        map.make_map(&mut ecs, &settings);
        let log = Rc::new(MessageLog::new());
        let fov_map = fov::initialize_fov(&map);
        let log_panel = MessagePanel::new(settings.message_pos(),
                                          settings.message_dimensions(),
                                          Rc::clone(&log));

        Game {
            ecs,
            map,
            log,
            settings,
            state: GameState::PlayersTurn,
            mouse_pos: (0, 0),
            fov_map,
            log_panel
        }
    }

    pub fn from_json(settings: &Settings, json: JsonValue) -> Game {

        let mut ecs = Ecs::deserialize(&json["ecs"]);
        let map = GameMap::deserialize(&json["map"]);
        let log = Rc::new(MessageLog::deserialize(&json["log"]));

        let fov_map = fov::initialize_fov(&map);
        let log_panel = MessagePanel::new(settings.message_pos(),
                                          settings.message_dimensions(),
                                          Rc::clone(&log));

        Game {
            ecs,
            map,
            log,
            settings,
            state: GameState::PlayersTurn,
            mouse_pos: (0, 0),
            fov_map,
            log_panel
        }
    }


    pub fn run(&mut self, root: &mut Root) {
        'game_loop: while !root.window_closed() {
            {
                let player_pos = self.ecs.get_component::<Position>(self.ecs.player_entity_id).unwrap();
                fov::recompute_fov(&mut self.fov_map, (player_pos.position.0, player_pos.position.1), &self.settings);
            }

            render_all(&self.ecs, root, &self.settings, &mut self.map, &self.fov_map,
                       &self.state, &self.log_panel, self.mouse_pos);

            let result = self.state.run(&mut self.ecs, &self.fov_map, Rc::clone(&self.log), &self.map);

            if let Some(engine_action) = result.engine_action {
                match engine_action {
                    EngineAction::Exit(save) => {
                            match save {
                                true => savegame::save(self),
                                false => savegame::delete()
                            }
                            break 'game_loop
                        }
                    EngineAction::ToggleFullscreen => {
                        let fullscreen = root.is_fullscreen();
                        root.set_fullscreen(!fullscreen)
                    },
                    EngineAction::MousePos(x, y) => {
                        self.mouse_pos.0 = x as i32;
                        self.mouse_pos.1 = y as i32;
                    }
                }
            }

            self.state = result.next_state;
        }
    }
}

impl <'a> Serialize for Game <'a> {
    fn serialize(&self) -> JsonValue {
        object!(
            "ecs" => self.ecs.serialize(),
            "log" => self.log.serialize(),
            "map" => self.map.serialize(),
        )
    }
}