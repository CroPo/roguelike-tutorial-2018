use std::rc::Rc;
use std::ops::DerefMut;
use std::ops::Deref;

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
use ecs::component::Position;

pub mod state;
pub mod input;

pub struct Game {
    pub ecs: RefCell<Ecs>,
    pub map: RefCell<GameMap>,
    pub log: Rc<MessageLog>,

    pub fov_map: RefCell<Map>,
    pub log_panel: MessagePanel,
    pub floor_number: u8
}

impl Game {
    pub fn new() -> Game {

        let mut ecs = Ecs::initialize();
        let mut map = GameMap::new(1, 1);
        let log = Rc::new(MessageLog::new());
        let fov_map = Map::new(1,1);
        let log_panel = MessagePanel::new((0,0),(0,0),Rc::clone(&log));

        Game {
            ecs: RefCell::new(ecs),
            map: RefCell::new(map),
            log,
            fov_map: RefCell::new(fov_map),
            log_panel,
            floor_number: 1
        }
    }

    pub fn start_new(&mut self, settings: &Settings) {

        let mut ecs = Ecs::initialize();
        let mut map = GameMap::new(settings.map_width(), settings.map_height());
        map.make_map(&mut ecs, &settings, self.floor_number);
        let log = MessageLog::new();
        let fov_map = fov::initialize_fov(&map);

        self.ecs = RefCell::new(ecs);
        self.map = RefCell::new(map);
        self.log = Rc::new(log);
        self.fov_map = RefCell::new(fov_map);
        self.log_panel = MessagePanel::new(settings.message_pos(),
                                           settings.message_dimensions(),
                                           self.log.clone());
    }

    pub fn load(&mut self, settings: &Settings, json: JsonValue) {

        let ecs = Ecs::deserialize(&json["ecs"]);
        let map = GameMap::deserialize(&json["map"]);
        let log = MessageLog::deserialize(&json["log"]);

        let fov_map = fov::initialize_fov(&map);

        self.ecs = RefCell::new(ecs);
        self.map = RefCell::new(map);
        self.floor_number = json["floor_number"].as_u8().unwrap();
        self.log = Rc::new(log);
        self.fov_map = RefCell::new(fov_map);
        self.log_panel = MessagePanel::new(settings.message_pos(),
                                           settings.message_dimensions(),
                                           self.log.clone());
    }

    pub fn next_floor(&mut self, settings: &Settings) {

        self.floor_number+=1;
        let mut ecs = self.ecs.borrow_mut();

        ecs.get_all_ids::<Position>().iter().for_each(|id|{
            if *id != ecs.player_entity_id {
                ecs.destroy_entity(id);
            }
        });

        self.map.borrow_mut().make_map(ecs.deref_mut(), settings, self.floor_number);
        self.fov_map = RefCell::new(fov::initialize_fov(&self.map.borrow()));
    }

}

impl Serialize for Game {
    fn serialize(&self) -> JsonValue {
        object!(
            "ecs" => self.ecs.borrow().serialize(),
            "log" => self.log.serialize(),
            "map" => self.map.borrow().serialize(),
            "floor_number" => self.floor_number
        )
    }
}