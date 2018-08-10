use json::JsonValue;

use savegame::{Serialize, Deserialize};

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

impl Deserialize for Tile {
    fn deserialize(json: &JsonValue) -> Self {
        Tile {
            block_move: json[0].as_bool().unwrap(),
            block_sight: json[1].as_bool().unwrap(),
            explored: json[2].as_bool().unwrap(),
        }
    }
}