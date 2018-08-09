use std::rc::Rc;

use tcod::Map;

use json::JsonValue;

use ecs::Ecs;

use map_objects::map::GameMap;
use message::MessageLog;
use settings::Settings;
use render::MessagePanel;
use map_objects::fov;
use savegame::{Serialize, Deserialize};
use std::cell::RefCell;

pub mod state;
pub mod input;

pub struct Game {
    pub ecs: RefCell<Ecs>,
    pub map: RefCell<GameMap>,
    pub log: Rc<MessageLog>,

    pub fov_map: RefCell<Map>,
    pub log_panel: MessagePanel
}

impl Game {
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
            ecs: RefCell::new(ecs),
            map: RefCell::new(map),
            log,
            fov_map: RefCell::new(fov_map),
            log_panel
        }
    }

    pub fn from_json(settings: &Settings, json: JsonValue) -> Game {

        let ecs = Ecs::deserialize(&json["ecs"]);
        let map = GameMap::deserialize(&json["map"]);
        let log = Rc::new(MessageLog::deserialize(&json["log"]));

        let fov_map = fov::initialize_fov(&map);
        let log_panel = MessagePanel::new(settings.message_pos(),
                                          settings.message_dimensions(),
                                          Rc::clone(&log));

        Game {
            ecs: RefCell::new(ecs),
            map: RefCell::new(map),
            log,
            fov_map: RefCell::new(fov_map),
            log_panel
        }
    }
}

impl Serialize for Game {
    fn serialize(&self) -> JsonValue {
        object!(
            "ecs" => self.ecs.borrow().serialize(),
            "log" => self.log.serialize(),
            "map" => self.map.borrow().serialize(),
        )
    }
}