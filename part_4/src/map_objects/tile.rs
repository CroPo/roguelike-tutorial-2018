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