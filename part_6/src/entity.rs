use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use render::Render;
use components::fighter::Fighter;
use components::ai::Ai;

type EntityId = u8;
/// Generator for IDs which are used to identify an `Entity`
struct IdGenerator {
    id: EntityId
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator {
            id: 0,
        }
    }

    /// Generate a new ID
    fn get_next_id(&mut self) -> EntityId {
        self.id+=1;
        self.id
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
    pub fn new(x: i32, y: i32, glyph: char, color: colors::Color, name: String, fighter : Option<Fighter>, ai: Option<Box<Ai>>) -> Entity {
        Entity {
            pos: (x, y),
            glyph,
            color,
            name,
            blocks: true
        }
    }

    pub fn mv(&mut self, d_pos: (i32, i32)) {
        self.pos.0 += d_pos.0;
        self.pos.1 += d_pos.1;
    }

    pub fn get_blocking_entities_at(entities: &Vec<Self>, x: i32, y: i32) -> Vec<&Entity> {
        entities.iter().filter(|e| e.blocks && e.pos.0 == x && e.pos.1 == y).collect()
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