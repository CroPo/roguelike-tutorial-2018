use json::JsonValue;
use json;

use savegame::Serialize;

#[derive(Clone)]
pub struct Tile {
    pub block_move: bool,
    pub block_sight: bool,
    pub explored: bool
}

impl Tile {
    pub fn new(block_move: bool, block_sight: bool) -> Self {

        Tile {
            block_move, block_sight, explored : false
        }
    }
}

impl Serialize for Tile {
    fn serialize(&self) -> JsonValue {
        array![self.block_move, self.block_sight, self.explored]
    }
}