use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use render::Render;


/// A generic representation of things like NPCs, Monsters, Items, ... and of course, of the player, in the game.
pub struct Entity {
    pos: (i32, i32),
    glyph: char,
    color: colors::Color
}

impl Entity {

    pub fn new(x: i32, y:i32, glyph: char, color: colors::Color) -> Entity{
        Entity {
            pos: (x, y),
            glyph,
            color
        }
    }

    pub fn mv(&mut self, d_pos: (i32, i32)) {
        self.pos.0 += d_pos.0;
        self.pos.1 += d_pos.1;
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