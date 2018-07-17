use std::any::Any;
use ecs::id::EntityId;
use ecs::Ecs;

use tcod::colors::Color;
use tcod::Console;
use tcod::BackgroundFlag;
use ecs::action::EntityAction;

/// Used to indentify an Component
pub trait Component: Any {}

/// A Component which contains informations of an `Entity`s position on the Map, and methods to
/// interact with it
pub struct Position {
    pub position: (i32, i32),
    pub is_blocking: bool,
}

impl Position {
    pub fn new(is_blocking: bool) -> Position {
        Position {
            position: (0, 0),
            is_blocking,
        }
    }

    /// Checks if a position is already blocked by an `Entity` and returns the id of the blocker.
    pub fn is_blocked_by(ecs: &Ecs, position: (i32, i32)) -> Vec<EntityId> {
        ecs.get_all::<Self>().iter().filter(|(_, p)| {
            p.position.0 == position.0 && p.position.1 == position.1
        }).map(|(i, _)| *i).collect()
    }

    /// Change the Position of the Entity
    pub fn mv(&mut self, vel: (i32, i32)) {
        self.position.0 += vel.0;
        self.position.1 += vel.1;
    }
}

impl Component for Position {}


/// This component handles the rendering of an Entity onto the map
pub struct Render {
    glyph: char,
    color: Color,
}

impl Render {

    pub fn new(glyph : char, color : Color ) -> Render {
        Render {
            glyph, color
        }
    }

    pub fn draw(&self, console: &mut Console, pos: (i32, i32)) {
        console.set_default_foreground(self.color);
        console.put_char(pos.0, pos.1, self.glyph, BackgroundFlag::None);
    }

    pub fn clear(&self, console: &mut Console, pos: (i32, i32)) {
        console.put_char(pos.0, pos.1, ' ', BackgroundFlag::None);
    }
}

impl Component for Render {}

/// The name and other textual data refering to an entity
pub struct Name {
    pub name : String,
    pub description: String
}

impl Component for Name {}


/// Basic stats for any creature
pub struct Creature {
    max_hp : i32,
    hp: i32,
    power: i32,
    defense: i32
}

impl Creature {
    pub fn new(max_hp: i32, power: i32, defense: i32) -> Creature {
        Creature {
            max_hp, hp: max_hp, power, defense
        }
    }
}

impl Component for Creature {}

pub struct MonsterAi {
    entity_id: EntityId
}

impl MonsterAi {
    pub fn new(entity_id : EntityId) -> MonsterAi {
        MonsterAi {entity_id}
    }

    pub fn calculate_turn(&self, ecs: &Ecs) -> EntityAction {
        match ecs.get_component::<Name>(self.entity_id) {
            Some(n) => {
                println!("The {} can't wait to finally move", n.name)
            }
            _ => ()
        }
        EntityAction::Idle
    }
}

impl Component for MonsterAi {}
