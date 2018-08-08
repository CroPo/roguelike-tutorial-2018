use std::any::Any;
use ecs::id::EntityId;
use ecs::Ecs;

use tcod::colors::Color;
use tcod::Console;
use tcod::BackgroundFlag;
use tcod::map::Map;
use tcod::pathfinding::AStar;

use json::JsonValue;
use json;

use ecs::action::EntityAction;
use map_objects::map::GameMap;
use render::RenderOrder;
use ecs::spell::Spell;

use savegame::{Serialize, Deserialize};

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
        ecs.get_all::<Self>().iter().filter(|(_, p)| {
            p.position.0 == position.0 && p.position.1 == position.1 && p.is_blocking
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

        ecs.get_all::<Position>().iter().filter(|(id, _)| {
            **id != target_id && **id != self.entity_id
        }).for_each(|(_, p)| {
            fov.set(p.position.0, p.position.1, true, false);
        });

        let mut path = AStar::new_from_map(fov, 1.41);
        path.find((self.position.0, self.position.1), (target.position.0, target.position.1));

        return if !path.is_empty() && path.len() < 25 {
            path.iter().next()
        } else {
            self.calculate_move_towards(ecs, map, (target.position.0, target.position.1))
        };
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

    pub fn clear(&self, ecs: &Ecs, console: &mut Console) {
        if let Some(p) = ecs.get_component::<Position>(self.entity_id) {
            console.put_char(p.position.0, p.position.1, ' ', BackgroundFlag::None);
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
}

impl Actor {
    pub fn new(entity_id: EntityId, max_hp: u32, power: i32, defense: i32) -> Actor {
        Actor {
            entity_id,
            max_hp,
            hp: max_hp,
            power,
            defense,
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
        }
    }
}

impl Component for Actor {}

pub struct MonsterAi {
    entity_id: EntityId,
    target_id: Option<EntityId>
}

impl MonsterAi {
    pub fn new(entity_id: EntityId) -> MonsterAi {
        MonsterAi { entity_id, target_id:None }
    }

    pub fn set_target(&mut self, target_id: EntityId) {
        self.target_id = Some(target_id)
    }

    pub fn has_no_target(&self) -> bool {
        return self.target_id.is_none()
    }

    pub fn calculate_turn(&self, ecs: &Ecs, map: &GameMap) -> EntityAction {
        if self.target_id.is_none() {
            EntityAction::Idle
        } else {
            match ecs.get_component::<Position>(self.entity_id) {
                Some(monster_position) => {
                    self.calculate_movement(ecs, monster_position, map)
                }
                _ => EntityAction::Idle
            }
        }
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
            "target" => self.target_id
            )
        )
    }
}

impl Deserialize for MonsterAi {
    fn deserialize(json: &JsonValue) -> Self {
        MonsterAi {
            entity_id: json["id"].as_u16().unwrap(),
            target_id: json["target"].as_u16()
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
    fn deserialize(json: &JsonValue) -> Self {
        Corpse {
        }
    }
}


impl Component for Corpse {}

pub struct Item {
    spell: Spell
}

impl Item {
    pub fn new(spell: Spell) -> Item {
        Item {
            spell
        }
    }

    pub fn use_item(&self) -> Option<Spell> {
        Some(self.spell.clone())
    }
}

impl Serialize for Item {
    fn serialize(&self) -> JsonValue {

        object!(
        "type" => "Item",
        "data" => object!(
                "spell" => self.spell.serialize(),
            )
        )
    }
}

impl Deserialize for Item {
    fn deserialize(json: &JsonValue) -> Self {
        Item {
            spell: Spell::deserialize(&json["spell"]),
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
