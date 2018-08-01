use ecs::Ecs;
use map_objects::map::GameMap;
use message::MessageLog;
use std::rc::Rc;
use settings::Settings;
use game::state::GameState;
use tcod::Map;
use render::MessagePanel;
use map_objects::fov;
use tcod::console::Root;
use ecs::component::Position;
use render::render_all;
use serde_json::to_string;
use savegame;

pub mod state;
pub mod input;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    Exit(bool),
}

#[derive(Serialize)]
pub struct Game <'a> {
    ecs: Ecs,
    map: GameMap,
    log: Rc<MessageLog>,

    #[serde(skip)]
    settings: &'a Settings,
    #[serde(skip)]
    state: GameState,
    #[serde(skip)]
    mouse_pos: (i32, i32),
    #[serde(skip)]
    fov_map: Map,
    #[serde(skip)]
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
                                true => savegame::write(&self),
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
