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
use ecs::component::MonsterAi;

pub mod state;
pub mod input;

pub struct Game<'game> {
    pub ecs: RefCell<Ecs>,
    pub map: RefCell<GameMap>,
    pub log: Rc<MessageLog>,

    pub settings: &'game Settings,

    pub fov_map: RefCell<Map>,
    pub log_panel: MessagePanel,
    pub floor_number: u8
}

impl<'game> Game<'game> {
    pub fn new(settings: &'game Settings) -> Game {

        let ecs = Ecs::initialize();
        let map = GameMap::new(1, 1);
        let log = Rc::new(MessageLog::new());
        let fov_map = Map::new(1,1);
        let log_panel = MessagePanel::new((0,0),(0,0),Rc::clone(&log));

        Game {
            ecs: RefCell::new(ecs),
            map: RefCell::new(map),
            log,
            settings,
            fov_map: RefCell::new(fov_map),
            log_panel,
            floor_number: 1
        }
    }

    pub fn start_new(&mut self) {

        let mut ecs = Ecs::initialize();
        let mut map = GameMap::new(self.settings.map_width(), self.settings.map_height());
        map.make_map(&mut ecs, self.settings, self.floor_number);
        let log = MessageLog::new();
        let fov_map = fov::initialize_fov(&map);

        self.ecs = RefCell::new(ecs);
        self.map = RefCell::new(map);
        self.log = Rc::new(log);
        self.fov_map = RefCell::new(fov_map);
        self.log_panel = MessagePanel::new(self.settings.message_pos(),
                                           self.settings.message_dimensions(),
                                           self.log.clone());

        self.init_entities(self.ecs.borrow_mut().deref_mut());
    }

    pub fn load(&mut self, json: JsonValue) {

        let ecs = Ecs::deserialize(&json["ecs"]);
        let map = GameMap::deserialize(&json["map"]);
        let log = MessageLog::deserialize(&json["log"]);

        let fov_map = fov::initialize_fov(&map);

        self.ecs = RefCell::new(ecs);
        self.map = RefCell::new(map);
        self.floor_number = json["floor_number"].as_u8().unwrap();
        self.log = Rc::new(log);
        self.fov_map = RefCell::new(fov_map);
        self.log_panel = MessagePanel::new(self.settings.message_pos(),
                                           self.settings.message_dimensions(),
                                           self.log.clone());

        self.init_entities(self.ecs.borrow_mut().deref_mut());
    }

    pub fn next_floor(&mut self) {

        self.floor_number+=1;
        let mut ecs = self.ecs.borrow_mut();

        ecs.get_all_ids::<Position>().iter().for_each(|id|{
            if *id != ecs.player_entity_id {
                ecs.destroy_entity(id);
            }
        });

        self.map.borrow_mut().make_map(ecs.deref_mut(), self.settings, self.floor_number);
        self.fov_map = RefCell::new(fov::initialize_fov(&self.map.borrow()));

        self.init_entities(ecs.deref_mut());
    }

    /// run initialization on the entities
    fn init_entities(&self, ecs : &mut Ecs) {
        ecs.get_all_ids::<MonsterAi>().clone().iter().for_each(|id| {
            if let Some(ai) = ecs.get_component_mut::<MonsterAi>(*id) {
                ai.initialize_fov(&self.map.borrow())
            }
        })
    }

}

impl<'game> Serialize for Game<'game> {
    fn serialize(&self) -> JsonValue {
        object!(
            "ecs" => self.ecs.borrow().serialize(),
            "log" => self.log.serialize(),
            "map" => self.map.borrow().serialize(),
            "floor_number" => self.floor_number
        )
    }
}