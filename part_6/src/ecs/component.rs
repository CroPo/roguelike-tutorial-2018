use std::any::Any;
use ecs::id::EntityId;
use ecs::Ecs;

use tcod::colors::Color;
use tcod::Console;
use tcod::BackgroundFlag;
use tcod::map::Map;
use tcod::pathfinding::AStar;

use ecs::action::EntityAction;
use map_objects::map::GameMap;
use render::RenderOrder;

/// Used to indentify an Component
pub trait Component: Any {}

/// A Component which contains informations of an `Entity`s position on the Map, and methods to
/// interact with it
pub struct Position {
    entity_id: EntityId,
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

impl Component for Render {}

/// The name and other textual data refering to an entity
pub struct Name {
    pub name: String,
    pub description: String,
}

impl Component for Name {}


/// Basic stats for any creature
pub struct Creature {
    entity_id: EntityId,
    pub max_hp: i32,
    pub hp: i32,
    pub power: i32,
    pub defense: i32,
}

impl Creature {
    pub fn new(entity_id: EntityId, max_hp: i32, power: i32, defense: i32) -> Creature {
        Creature {
            entity_id,
            max_hp,
            hp: max_hp,
            power,
            defense,
        }
    }

    /// Take a specific amount of damage.
    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    /// Calculate the Attack and return the amount of damage which will be dealt
    pub fn calculate_attack(&self, ecs: &Ecs, target_id: EntityId) -> Option<i32> {
        if let Some(target) = ecs.get_component::<Creature>(target_id) {
            let entity_name = match ecs.get_component::<Name>(self.entity_id) {
                Some(n) => n.name.to_uppercase(),
                None => "AN UNNAMED ENTITY".to_string()
            };

            let target_name = match ecs.get_component::<Name>(target_id) {
                Some(n) => n.name.clone(),
                None => "an unnamed entity".to_string()
            };

            let damage = self.power - target.defense;
            if damage > 0 {
                println!("{} attacks {} for {} hit points.", entity_name, target_name, damage);
                return Some(damage);
            }
            println!("{} attacks {} but does no damage.", entity_name, target_name);
        };
        None
    }
}

impl Component for Creature {}

pub struct MonsterAi {
    entity_id: EntityId
}

impl MonsterAi {
    pub fn new(entity_id: EntityId) -> MonsterAi {
        MonsterAi { entity_id }
    }

    pub fn calculate_turn(&self, ecs: &Ecs, map: &GameMap) -> EntityAction {
        match ecs.get_component::<Position>(self.entity_id) {
            Some(monster_position) => {
                self.calculate_movement(ecs, monster_position, map)
            }
            _ => EntityAction::Idle
        }
    }

    fn calculate_movement(&self, ecs: &Ecs, monster_position: &Position, map: &GameMap) -> EntityAction {
        match ecs.get_component::<Position>(ecs.player_entity_id) {
            Some(player_position) => {
                let target = (player_position.position.0, player_position.position.1);
                let distance = monster_position.distance_to(target);

                if distance >= 2.0 {
                    match monster_position.calculate_move_astar(ecs, map, ecs.player_entity_id) {
                        Some(pos) => return EntityAction::MoveTo(self.entity_id, pos),
                        _ => ()
                    }
                } else {
                    return self.calculate_attack(ecs);
                }

                EntityAction::Idle
            }
            _ => EntityAction::Idle
        }
    }

    fn calculate_attack(&self, ecs: &Ecs) -> EntityAction {
        match ecs.get_component::<Creature>(self.entity_id) {
            Some(c) => {
                match c.calculate_attack(ecs, ecs.player_entity_id) {
                    Some(damage) => EntityAction::TakeDamage(ecs.player_entity_id, damage),
                    None => EntityAction::Idle
                }
            }
            _ => EntityAction::Idle
        }
    }
}

impl Component for MonsterAi {}

pub struct Corpse {}

impl Component for Corpse {}
