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

/// Handling access to Entities and to their Components
pub struct Ecs {
    id_generator: IdGenerator,
    pub entities: HashMap<EntityId, Entity>,
    player_entity_id: EntityId,
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            id_generator: IdGenerator::new(),
            entities: HashMap::new(),
            player_entity_id: 0,
        }
    }

    /// Add a creature from a template to a specific position
    pub fn add_creature(&mut self, template: CreatureTemplate, position: (i32, i32)) {
        if template.is_player() && self.player_entity_id != 0 {
            // Player Entity has already been created. Abort
            return;
        }

        match template.create() {
            Some(entity) => {
                let id = self.id_generator.get_next_id();
                self.entities.insert(id, entity);

                self.entities.get_mut(&id).unwrap().pos = position;

                if template.is_player() {
                    self.player_entity_id = id;
                }
            }
            None => ()
        }
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
            .map(|(_, e)| e )
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