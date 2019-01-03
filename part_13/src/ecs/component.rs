use std::any::Any;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

use ecs::id::EntityId;
use ecs::Ecs;

use tcod::colors::Color;
use tcod::Console;
use tcod::BackgroundFlag;
use tcod::map::Map;
use tcod::pathfinding::AStar;

use json::JsonValue;

use ecs::action::EntityAction;
use map_objects::map::GameMap;
use render::RenderOrder;
use ecs::spell::Spell;

use savegame::{Serialize, Deserialize};
use map_objects::fov::initialize_fov;
use map_objects::fov::recompute_fov;
use settings::Settings;
use std::collections::HashMap;

/// Used to indentify an Component
pub trait Component: Any + Serialize {}

/// A Component which contains informations of an `Entity`s position on the Map, and methods to
/// interact with it
#[derive(Clone)]
pub struct Position {
    pub entity_id: EntityId,
    pub position: (i32, i32),
    pub is_blocking: bool,
}

impl Position {
    pub fn new(entity_id: EntityId, is_blocking: bool) -> Position {
        Position {
            entity_id,
            position: (0, 0),
            is_blocking,
        }
    }

    /// Checks if a position is already blocked by an `Entity` and returns the id of the blocker.
    pub fn is_blocked_by(ecs: &Ecs, position: (i32, i32)) -> Vec<EntityId> {
        ecs.get_all::<Self>().iter().filter(|(entity_id, p)| {
            let is_blocking = p.position.0 == position.0 && p.position.1 == position.1 && p.is_blocking;
            if let Some(a) = ecs.get_component::<Actor>(**entity_id) {
                is_blocking && !a.is_dead()
            } else {
                is_blocking
            }
        }).map(|(i, _)| *i).collect()
    }

    /// Change the Position of the Entity relative to its current position
    pub fn move_relative(&mut self, delta: (i32, i32)) {
        self.position.0 += delta.0;
        self.position.1 += delta.1;
    }

    /// Change the Position of the Entity to a fixed point.
    pub fn move_absolute(&mut self, pos: (i32, i32)) {
        self.position = pos;
    }

    /// Calculate the distance to a specific point
    pub fn distance_to(&self, target: (i32, i32)) -> f64 {
        let dx = (target.0 - self.position.0) as f64;
        let dy = (target.1 - self.position.1) as f64;

        (dx * dx + dy * dy).sqrt()
    }

    /// Simple movement calculation
    pub fn calculate_move_towards(&self, ecs: &Ecs, map: &GameMap, target: (i32, i32)) -> Option<(i32, i32)> {
        let mut dx = (target.0 - self.position.0) as f64;
        let mut dy = (target.1 - self.position.1) as f64;

        let distance = (dx * dx + dy * dy).sqrt();

        dx = (dx / distance).round();
        dy = (dy / distance).round();

        let vel = (dx as i32, dy as i32);
        let target = (self.position.0 + vel.0, self.position.1 + vel.1);

        if map.is_move_blocked(target.0, target.1) || !Self::is_blocked_by(ecs, target).is_empty() {
            return None;
        }
        Some(target)
    }

    /// Calculate the next movement step with A*
    pub fn calculate_move_astar(&self, ecs: &Ecs, map: &GameMap, target_id: EntityId) -> Option<(i32, i32)> {
        let target = match ecs.get_component::<Position>(target_id) {
            Some(p) => p,
            _ => return None
        };

        let mut fov = Map::new(map.dimensions.0, map.dimensions.1);

        for x in 0..map.dimensions.0 {
            for y in 0..map.dimensions.1 {
                let tile = map.get_tile(x as usize, y as usize);
                fov.set(x, y, !tile.block_sight, !tile.block_move);
            }
        }

        ecs.get_all::<Position>().iter().filter(|(id, p)| {
            // Filter out all entities which can be ignored for pathfinding:
            // - the entity itself
            // - dead actors
            // - non-blocking entities
            let is_self = **id != target_id && **id != self.entity_id;
            let is_blocking = p.is_blocking;
            if let Some(a) = ecs.get_component::<Actor>(**id) {
                is_self && is_blocking && !a.is_dead()
            } else {
                is_self && is_blocking
            }

        }).for_each(|(_, p)| {
            fov.set(p.position.0, p.position.1, true, !p.is_blocking);
        });

        let mut path = AStar::new_from_map(fov, 1.41);
        path.find((self.position.0, self.position.1), (target.position.0, target.position.1));

        if !path.is_empty() && path.len() < 25 {
            path.iter().next()
        } else {
            self.calculate_move_towards(ecs, map, (target.position.0, target.position.1))
        }
    }

    pub fn x(&self) -> i32 {
        self.position.0
    }

    pub fn y(&self) -> i32 {
        self.position.1
    }
}

impl Serialize for Position {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Position",
        "data" => object!(
                "id" => self.entity_id,
                "x" => self.position.0,
                "y" => self.position.1,
                "blocking" => self.is_blocking,
            )
        )
    }
}

impl Deserialize for Position {
    fn deserialize(json: &JsonValue) -> Self {
        Position {
            entity_id: json["id"].as_u16().unwrap(),
            position: (json["x"].as_i32().unwrap(), json["y"].as_i32().unwrap()),
            is_blocking: json["blocking"].as_bool().unwrap()
        }
    }
}

impl Component for Position {}


/// This component handles the rendering of an Entity onto the map
pub struct Render {
    pub entity_id: EntityId,
    glyph: char,
    color: Color,
    pub order: RenderOrder,
}

impl Render {
    pub fn new(entity_id: EntityId, glyph: char, color: Color, order: RenderOrder) -> Render {
        Render {
            entity_id,
            glyph,
            color,
            order,
        }
    }

    pub fn draw(&self, ecs: &Ecs, console: &mut Console) {
        if let Some(p) = ecs.get_component::<Position>(self.entity_id) {
            console.set_default_foreground(self.color);
            console.put_char(p.position.0, p.position.1, self.glyph, BackgroundFlag::None);
        }
    }
}


impl Serialize for Render {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Render",
        "data" => object!(
            "id" => self.entity_id,
            "glyph" => self.glyph.to_string(),
            "order" => self.order.to_string(),
            "color" => array![self.color.r, self.color.g, self.color.b]
            )
        )
    }
}

impl Deserialize for Render {
    fn deserialize(json: &JsonValue) -> Self {
        Render {
            entity_id: json["id"].as_u16().unwrap(),
            glyph: json["glyph"].as_str().unwrap().chars().next().unwrap(),
            order: RenderOrder::deserialize(&json["order"]),
            color: Color {
                r : json["color"][0].as_u8().unwrap(),
                g : json["color"][1].as_u8().unwrap(),
                b : json["color"][2].as_u8().unwrap(),
            }
        }
    }
}


impl Component for Render {}

/// The name and other textual data refering to an entity
pub struct Name {
    pub name: String
}

impl Serialize for Name {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Name",
        "data" => object!(
                "name" => self.name.clone(),
            )
        )
    }
}

impl Deserialize for Name {
    fn deserialize(json: &JsonValue) -> Self {
        Name {
            name: json["name"].as_str().unwrap().to_string(),
        }
    }
}

impl Component for Name {}


/// Basic stats for any creature
pub struct Actor {
    entity_id: EntityId,
    pub max_hp: u32,
    pub hp: u32,
    pub power: i32,
    pub defense: i32,
    pub xp_reward: u32,
}

impl Actor {
    pub fn new(entity_id: EntityId, max_hp: u32, power: i32, defense: i32, xp_reward: u32) -> Actor {
        Actor {
            entity_id,
            max_hp,
            hp: max_hp,
            power,
            defense,
            xp_reward,
        }
    }

    /// Take a specific amount of damage.
    pub fn take_damage(&mut self, damage: u32) {
        if self.hp < damage {
            self.hp = 0;
        } else {
            self.hp -= damage;
        }
    }

    /// Calculate the Attack and return the amount of damage which will be dealt
    pub fn calculate_attack(&self, ecs: &Ecs, target_id: EntityId) -> Option<u32> {
        if let Some(target) = ecs.get_component::<Actor>(target_id) {
            let mut damage = self.power - target.defense;
            if damage < 0 {
                damage = 0;
            }
            Some(damage as u32)

        } else {
            None
        }
    }

    pub fn is_dead(&self) -> bool{
        self.hp == 0
    }
}

impl Serialize for Actor {
    fn serialize(&self) -> JsonValue {

        object!(
        "type" => "Actor",
        "data" => object!(
                "id" => self.entity_id,
                "max_hp" => self.max_hp,
                "hp" => self.hp,
                "power" => self.power,
                "defense" => self.defense,
                "xp_reward" => self.xp_reward,
            )
        )
    }
}

impl Deserialize for Actor {
    fn deserialize(json: &JsonValue) -> Self {
        Actor {
            entity_id: json["id"].as_u16().unwrap(),
            max_hp: json["max_hp"].as_u32().unwrap(),
            hp: json["hp"].as_u32().unwrap(),
            power: json["power"].as_i32().unwrap(),
            defense: json["defense"].as_i32().unwrap(),
            xp_reward: json["xp_reward"].as_u32().unwrap(),
        }
    }
}

impl Component for Actor {}

pub struct MonsterAi {
    entity_id: EntityId,
    target_id: Option<EntityId>,
    fov_map: Map,
    chase_target: bool
}

impl MonsterAi {
    pub fn new(entity_id: EntityId) -> MonsterAi {

        MonsterAi {
            entity_id,
            target_id:None,
            fov_map: Map::new(1,1),
            chase_target: false
        }
    }

    pub fn set_target(&mut self, target_id: EntityId) {
        self.target_id = Some(target_id)
    }

    pub fn has_no_target(&self) -> bool {
        return self.target_id.is_none()
    }

    pub fn calculate_turn(&self, ecs: &Ecs, map: &GameMap, settings: &Settings) -> EntityAction {

        if !self.is_within_ai_distance(ecs, settings) {
            EntityAction::Idle
        } else if self.target_id.is_none() {
            EntityAction::Idle
        } else {
            match (self.chase_target, ecs.get_component::<Position>(self.entity_id)) {
                (true, Some(monster_position)) => {
                    self.calculate_movement(ecs, monster_position, map)
                }
                _ => EntityAction::Idle
            }
        }
    }

    pub fn is_chasing_target(&self) -> bool {
        self.chase_target
    }


    pub fn set_chasing_target(&mut self, chase: bool) {
        self.chase_target = chase;
    }

    /// Returns true if the target is in FOV
    pub fn is_target_in_fov(&self, ecs : &Ecs, settings: &Settings) -> bool {
        if self.has_no_target() {
            false
        } else if let Some(player_position) = ecs.get_component::<Position>(self.target_id.unwrap()) {
            self.fov_map.is_in_fov(player_position.x(), player_position.y())
        } else {
            false
        }
    }


    /// return true if this `Entity` is within the defined AI distance to the player, or has no
    /// `Position` component at all.
    pub fn is_within_ai_distance(&self, ecs: &Ecs, settings: &Settings) -> bool {

        if let Some(positon) = ecs.get_component::<Position>(self.entity_id) {
            if let Some(player_position) = ecs.get_component::<Position>(ecs.player_entity_id) {
                player_position.distance_to(positon.position) <= settings.ai_distance()
            }
           else {
               false
           }
        } else {
            true
        }
    }


    pub fn initialize_fov(&mut self, game_map: &GameMap) {
        self.fov_map = Map::new(game_map.dimensions.0, game_map.dimensions.1);

        for x in 0..game_map.dimensions.0 {
            for y in 0..game_map.dimensions.1 {
                let tile = game_map.get_tile(x as usize, y as usize);
                self.fov_map.set(x, y, !tile.block_sight, !tile.block_move);
            }
        }
    }

    pub fn recompute_fov(&mut self, settings: &Settings, origin_x: i32, origin_y: i32) {
        self.fov_map.compute_fov(origin_x, origin_y,
                            settings.fov_radius(),
                            false,
                            settings.fov_algorithm());
    }

    fn calculate_movement(&self, ecs: &Ecs, monster_position: &Position, map: &GameMap) -> EntityAction {
        // Unwrap is safe here, because the `None` check has already been performed in `calculate_turn`.
        match ecs.get_component::<Position>(self.target_id.unwrap()) {
            Some(player_position) => {
                let target = (player_position.position.0, player_position.position.1);
                let distance = monster_position.distance_to(target);

                if distance >= 2.0 {
                    match monster_position.calculate_move_astar(ecs, map, self.target_id.unwrap()) {
                        Some(pos) => return EntityAction::MoveTo(self.entity_id, pos),
                        _ => ()
                    }
                } else {
                    return EntityAction::MeleeAttack(self.entity_id, self.target_id.unwrap())
                }

                EntityAction::Idle
            }
            _ => EntityAction::Idle
        }
    }
}

impl Serialize for MonsterAi {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "MonsterAi",
        "data" => object!(
            "id" => self.entity_id,
            "target" => self.target_id,
            "chase_target" => self.chase_target
            )
        )
    }
}

impl Deserialize for MonsterAi {
    fn deserialize(json: &JsonValue) -> Self {
        MonsterAi {
            entity_id: json["id"].as_u16().unwrap(),
            target_id: json["target"].as_u16(),
            fov_map: Map::new(1,1 ),
            chase_target: json["chase_target"].as_bool().unwrap_or(false)
        }
    }
}

impl Component for MonsterAi {}

pub struct Corpse {}

impl Serialize for Corpse {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Corpse",
        "data" => object!()
        )
    }
}

impl Deserialize for Corpse {
    fn deserialize(_json: &JsonValue) -> Self {
        Corpse {
        }
    }
}

impl Component for Corpse {}

pub struct Item {
    spell: Option<Spell>,
}

impl Item {
    pub fn consumable(spell: Spell) -> Self {
        Item {
            spell: Some(spell)
        }
    }

    pub fn equippable() -> Self {
        Item {
            spell: None
        }
    }

    pub fn use_item(&self) -> Option<Spell> {
        self.spell.clone()
    }
}

impl Serialize for Item {
    fn serialize(&self) -> JsonValue {

        let data = match self.spell {
            Some(spell) => object!("spell" => spell.serialize()),
            _ => JsonValue::Null
        };


        object!(
        "type" => "Item",
        "data" => data
        )
    }
}

impl Deserialize for Item {
    fn deserialize(json: &JsonValue) -> Self {
        Item {
            spell: if json["spell"].is_null() {
                None
            }  else {
                Some(Spell::deserialize(&json["spell"]))
            }
        }
    }
}

impl Component for Item {}

pub struct Inventory {
    max_items: usize,
    pub items: Vec<EntityId>
}

impl Inventory {
    pub fn new(max_items: usize) -> Inventory {
        Inventory {
            max_items,
            items: vec![]
        }
    }

    pub fn free_space(&self) -> usize {
        self.max_items - self.items.len()
    }

    pub fn add_item(&mut self, item_id: EntityId) {
        if self.items.len() < self.max_items {
            self.items.push(item_id);
        }
    }
    pub fn remove_item(&mut self, item_number: usize) {
        if self.items.len() > item_number {
            self.items.remove(item_number);
        }
    }
    pub fn remove_item_id(&mut self, item_id: EntityId) {

        let mut index_to_remove = 0;

        for item in self.items.iter() {
            if *item == item_id {
                break;
            }
            index_to_remove+=1;
        }

        self.remove_item(index_to_remove)
    }
}

impl Serialize for Inventory {
    fn serialize(&self) -> JsonValue {

        let mut items = JsonValue::new_array();
        self.items.iter().for_each(|id| {
            items.push(*id);
        });

        object!(
        "type" => "Inventory",
        "data" => object!(
                "max_items" => self.max_items,
                "items" => items
            )
        )

    }
}

impl Deserialize for Inventory {
    fn deserialize(json: &JsonValue) -> Self {
        let mut items = vec![];
        for item_json in json["items"].members() {
            items.push(item_json.as_u16().unwrap());
        }

        Inventory {
            max_items: json["max_items"].as_usize().unwrap(),
            items
        }
    }
}

impl Component for Inventory {}

pub struct Stairs {}

impl Serialize for Stairs {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Stair",
        "data" => object!()
        )
    }
}

impl Deserialize for Stairs {
    fn deserialize(_json: &JsonValue) -> Self {
        Stairs {
        }
    }
}

impl Component for Stairs {}

pub struct Level {
    entity_id: EntityId,
    base: u32,
    factor: f32,
    pub level: u8,
    pub xp_total: u32
}

impl Level {
    pub fn new(entity_id: EntityId, level: u8, base: u32, factor: f32) -> Self {
        Self {
            entity_id,
            xp_total: 0,
            level,
            base,
            factor
        }
    }

    pub fn xp_to_level(&self, level: i32) -> u32 {
        (self.base as f32 * (1.0 + self.factor).powi(level as i32 - 1)).floor() as u32
    }

    pub fn reward_xp(&mut self, xp: u32) -> bool {
        self.xp_total+=xp;
        let next_level = self.level as i32 + 1;
        let xp_to_next = self.xp_to_level(next_level);

        self.xp_total >= xp_to_next && xp_to_next > 0
    }

    pub fn level_up(&mut self) {
        self.level+=1;
    }
}

impl Serialize for Level {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Level",
            "data" => object!(
                "id" => self.entity_id,
                "xp" => self.xp_total,
                "level" => self.level,
                "base" => self.base,
                "factor" => self.factor,
            )
        )
    }
}

impl Deserialize for Level {
    fn deserialize(json: &JsonValue) -> Self {
        Level {
            entity_id: json["id"].as_u16().unwrap(),
            xp_total: json["xp"].as_u32().unwrap(),
            level: json["level"].as_u8().unwrap(),
            base: json["base"].as_u32().unwrap(),
            factor: json["factor"].as_f32().unwrap(),
        }
    }
}

impl Component for Level {}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Armor,
    None
}

impl Serialize for EquipmentSlot {
    fn serialize(&self) -> JsonValue {
        object!("type" => format!("{:?}",self))
    }
}

impl Deserialize for EquipmentSlot {
    fn deserialize(json: &JsonValue) -> Self {

        match json["type"].as_str().unwrap() {
            "MainHand" =>  EquipmentSlot::MainHand,
            "OffHand" =>  EquipmentSlot::OffHand,
            "Armor" =>  EquipmentSlot::Armor,
            _ => EquipmentSlot::None
        }
    }
}


pub struct Equippable {
    entity_id: EntityId,
    bonus_power: i32,
    bonus_defense: i32,
    bonus_max_hp: i32,
    slot: EquipmentSlot
}

impl Equippable {
    pub fn new(entity_id: EntityId, bonus_power: i32, bonus_defense: i32, bonus_max_hp: i32, slot: EquipmentSlot) -> Self {
        Equippable {
            entity_id, bonus_power, bonus_defense, bonus_max_hp, slot
        }
    }
}

impl Component for Equippable {}

impl Serialize for Equippable {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Equippable",
            "data" => object!(
                "id" => self.entity_id,
                "bonus_power" => self.bonus_power,
                "bonus_defense" => self.bonus_defense,
                "bonus_max_hp" => self.bonus_max_hp,
                "slot" => self.slot.serialize(),
            )
        )
    }
}

impl Deserialize for Equippable {
    fn deserialize(json: &JsonValue) -> Self {
        Equippable {
            entity_id: json["id"].as_u16().unwrap(),
            bonus_power: json["bonus_power"].as_i32().unwrap(),
            bonus_defense: json["bonus_power"].as_i32().unwrap(),
            bonus_max_hp: json["bonus_power"].as_i32().unwrap(),
            slot: EquipmentSlot::deserialize(&json["slot"]),
        }
    }
}

pub struct Equipment {
    entity_id: EntityId,
    pub slots: HashMap<EquipmentSlot, EntityId>
}

impl Equipment {
    pub fn new(entity_id: EntityId) -> Self {
        Equipment {
            entity_id,
            slots: HashMap::new()
        }
    }

    pub fn equip(&mut self, ecs: &Ecs, item_id: EntityId) {
        match ecs.get_component::<Equippable>(item_id) {
            Some(equippable) => { self.slots.insert(equippable.slot, item_id); },
            None => ()
        };
    }

    pub fn unequip(&mut self, ecs: &Ecs, item_id: EntityId) {
        if let Some(slot) = self.is_equipped(item_id) {
            self.slots.remove(&slot);
        }
    }

    pub fn is_equipped(&self, item_id: EntityId) -> Option<EquipmentSlot>{
        for (slot, item_in_slot) in &self.slots {
            if item_id == *item_in_slot {
                return Some(*slot);
            }
        }
        None
    }

}

impl Component for Equipment {}

impl Serialize for Equipment {

    fn serialize(&self) -> JsonValue {
        let mut slots = JsonValue::new_array();
        self.slots.iter().for_each(|(slot, entity_id)| {
            slots.push(object!(
                "slot" => slot.serialize(),
                "id" => *entity_id
            ));
        });

        object!(
        "type" => "Equipment",
            "data" => object!(
                "id" => self.entity_id,
                "slots" => slots
            )
        )
    }
}

impl Deserialize for Equipment {
    fn deserialize(json: &JsonValue) -> Self {

        let mut slots : HashMap<EquipmentSlot, EntityId> = HashMap::new();

        for entity_json in json["slots"].members() {

            let slot = EquipmentSlot::deserialize(&json["slot"]);
            let id = entity_json["id"].as_u16().unwrap();

            slots.insert(slot, id);
        }

        Equipment {
            entity_id: json["id"].as_u16().unwrap(),
            slots,
        }

    }
}


