pub mod id;
pub mod creature;

use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use render::Render;
use components::fighter::Fighter;
use components::ai::Ai;
use ecs::id::{IdGenerator, EntityId};
use std::collections::HashMap;
use ecs::creature::CreatureTemplate;
use std::any::TypeId;
use std::any::Any;

/// Used to indentify an Component
trait Component {}

struct C {

}

impl Component for C {}


struct EcsStorage {
    entity_id: EntityId,
    data: HashMap<TypeId, Box<Component>>,
}

impl EcsStorage {
    /// Register a component to the storage
    ///
    /// No special case handling if the component is already registered. The old component will be
    /// replaced by the new one without further notice.
    fn register<T>(&mut self, component: T) where T: Component + Any {
        self.data.insert(TypeId::of::<T>(), Box::new(component));
    }
}


/// Handling access to Entities and to their Components
pub struct Ecs {
    id_generator: IdGenerator,

    /// Id of the Entity which represents the player
    player_entity_id: EntityId,

    storage: HashMap<EntityId, EcsStorage>,

    pub entities: HashMap<EntityId, Entity>,
}

impl Ecs {
    pub fn initialize() -> Ecs {
        Ecs {
            id_generator: IdGenerator::new(),
            player_entity_id: 0,
            storage: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    /// Add an creature from a template to a specific position
    pub fn add_creature(&mut self, template: CreatureTemplate, position: (i32, i32)) {
        if template.is_player() && self.player_entity_id != 0 {
            // Player Entity has already been created. Abort
            return;
        }

        match template.create() {
            Some(entity) => {
                let id = self.id_generator.get_next_id();

                self.storage.insert(id, EcsStorage { entity_id: id, data: HashMap::new() });
                self.entities.insert(id, entity);

                self.entities.get_mut(&id).unwrap().pos = position;

                if template.is_player() {
                    self.player_entity_id = id;
                }
            }
            None => ()
        }
    }

    /// Register a component for a specific Entity.
    ///
    /// No Error handling if adding a Component to an Entity
    /// which doesn't exist.
    pub fn register_component<T>(&mut self, entity_id: EntityId, component: T)
        where T: Component + Any {
        self.storage.get_mut(&entity_id)
            .map(|storage| storage.add(component));
    }

    /// Get a borrow to the player `Entity`
    pub fn get_player(&self) -> Option<&Entity> {
        self.entities.get(&self.player_entity_id)
    }

    /// Get a mutable borrow to the player `Entity`
    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(&self.player_entity_id)
    }
}


/// A generic representation of things like NPCs, Monsters, Items, ... and of course, of the player, in the game.
pub struct Entity {
    pub pos: (i32, i32),
    glyph: char,
    color: colors::Color,
    pub name: String,
    blocks: bool,
}

impl Entity {
    pub fn new(x: i32, y: i32, glyph: char, color: colors::Color, name: String, fighter: Option<Fighter>, ai: Option<Box<Ai>>) -> Entity {
        Entity {
            pos: (x, y),
            glyph,
            color,
            name,
            blocks: true,
        }
    }

    pub fn mv(&mut self, d_pos: (i32, i32)) {
        self.pos.0 += d_pos.0;
        self.pos.1 += d_pos.1;
    }

    pub fn get_blocking_entities_at(ecs: &Ecs, x: i32, y: i32) -> Vec<&Entity> {
        ecs.entities.iter()
            .filter(|(_, e)| e.blocks && e.pos.0 == x && e.pos.1 == y)
            .map(|(_, e)| e)
            .collect()
    }
}

impl Render for Entity {
    fn draw(&self, console: &mut Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.pos.0, self.pos.1, self.glyph, BackgroundFlag::None);
    }

    fn clear(&self, console: &mut Console) {
        console.put_char(self.pos.0, self.pos.1, ' ', BackgroundFlag::None);
    }
}