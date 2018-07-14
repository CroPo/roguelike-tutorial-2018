pub mod id;
pub mod creature;
pub mod component;

use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use render::Render;

use ecs::id::{IdGenerator, EntityId};
use std::collections::HashMap;
use ecs::creature::CreatureTemplate;
use std::any::TypeId;
use std::any::Any;
use ecs::component::Component;
use ecs::component::Position;


struct EcsStorage {
    entity_id: EntityId,
    data: HashMap<TypeId, Box<Any>>,
}

impl EcsStorage {
    /// Register a component to the storage
    ///
    /// No special case handling if the component is already registered. The old component will be
    /// replaced by the new one without further notice.
    fn register<T>(&mut self, component: T)
        where T: Component + Any {
        self.data.insert(TypeId::of::<T>(), Box::new(component));
    }

    /// Check if a component is registered
    fn is_registered<T>(&self) -> bool
        where T: Component + Any {
        self.data.contains_key(&TypeId::of::<T>())
    }

    /// Get a specific component form the storage
    fn get<T>(&self) -> Option<&T>
        where T: Component + Any {
        self.data.get(&TypeId::of::<T>()).map(|e| e.downcast_ref()).unwrap()
    }
    /// Get a specific component form the storage
    fn get_mut<T>(&mut self) -> Option<&mut T>
        where T: Component + Any {
        self.data.get_mut(&TypeId::of::<T>()).map(|e| e.downcast_mut()).unwrap()
    }
}


/// Handling access to Entities and to their Components
pub struct Ecs {
    id_generator: IdGenerator,

    /// Id of the Entity which represents the player
    pub player_entity_id: EntityId,

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
                self.entities.insert(id, entity);

                self.storage.insert(id, EcsStorage { entity_id: id, data: HashMap::new() });

                self.register_component(id, Position::new(true));

                {
                    let pos_component = self.get_component_mut::<Position>(id);
                    if pos_component.is_some() {
                        pos_component.unwrap().position = position;
                    }
                }
                if template.is_player() {
                    self.player_entity_id = id;
                }
            }
            None => ()
        }
    }


    /// Get a reference to a `Component` of a specified entity
    pub fn get_component<T>(&self, entity_id: EntityId) -> Option<&T>
        where T: Component + Any {
        self.storage.get(&entity_id).map(|storage| {
            storage.get::<T>().unwrap()
        })
    }

    /// Get a mutable reference to a `Component` of a specified entity
    pub fn get_component_mut<T>(&mut self, entity_id: EntityId) -> Option<&mut T>
        where T: Component + Any {
        self.storage.get_mut(&entity_id).map(|storage| {
            storage.get_mut::<T>().unwrap()
        })
    }

    /// Get a `HashMap` of the specified `Component` indexed by the `EntitiyId`
    pub fn get_all<T: Component + Any>(&self) -> HashMap<EntityId, &T>
        where T: Component {
        let entity_ids = self.storage.keys().cloned();

        let mut component_map = HashMap::new();

        for entity_id in entity_ids.filter(|id| {
            self.has_component::<T>(*id)
        }) {
            component_map.insert(entity_id, self.get_component::<T>(entity_id).unwrap());
        }
        component_map
    }

    /// Register a component for a specific Entity.
    ///
    /// No Error handling if adding a Component to an Entity
    /// which doesn't exist.
    pub fn register_component<T>(&mut self, entity_id: EntityId, component: T)
        where T: Component {

        match self.storage.get_mut(&entity_id) {
            Some(storage) => {
                storage.register(component);
            }
            _ => {}
        }

    }

    /// Check if an Entity has a specific type
    pub fn has_component<T>(&self, entity_id: EntityId) -> bool
        where T: Component {
        let is_registered = self.storage.get(&entity_id)
            .map(|storage| storage.is_registered::<T>());

        if is_registered.is_some() {
            return is_registered.unwrap();
        }
        false
    }
}


/// A generic representation of things like NPCs, Monsters, Items, ... and of course, of the player, in the game.
pub struct Entity {
    glyph: char,
    color: colors::Color,
    pub name: String,
}

impl Entity {
    pub fn new(glyph: char, color: colors::Color, name: String) -> Entity {
        Entity {
            glyph,
            color,
            name,
        }
    }

    pub fn get_blocking_entities_at(ecs: &Ecs, x: i32, y: i32) -> Vec<&EntityId> {

        vec![]
    }
}

impl Render for Entity {
    fn draw(&self, console: &mut Console, ecs: &Ecs) {
        console.set_default_foreground(self.color);
        //console.put_char(self.pos.0, self.pos.1, self.glyph, BackgroundFlag::None);
    }

    fn clear(&self, console: &mut Console, ecs: &Ecs) {
        // console.put_char(self.pos.0, self.pos.1, ' ', BackgroundFlag::None);
    }
}