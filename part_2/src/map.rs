use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use tile::Tile;
use render::Render;

pub struct GameMap {
    dimensions: (i32, i32),
    tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new(width: i32, height: i32) -> GameMap {
        GameMap {
            dimensions: (width, height),
            tiles: Self::initialize_tiles(width as usize, height as usize),
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.dimensions.0 as usize + x]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.dimensions.0 as usize + x]
    }

    fn initialize_tiles(width: usize, height: usize) -> Vec<Tile> {
        let mut tiles = vec![Tile::new(false, false); height * width];

        tiles[width * 30 + 22] = Tile::new(true, true);
        tiles[width * 31 + 22].block_move = true;
        tiles[width * 31 + 22].block_sight = true;
        tiles[width * 32 + 22].block_move = true;
        tiles[width * 32 + 22].block_sight = true;

        tiles
    }
}

impl Render for GameMap {
    fn draw(&self, console: &mut Console) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let tile = self.get_tile(x as usize, y as usize);

                if tile.block_move {
                    console.set_char_background(x, y, colors::Color { r: 0, g: 0, b: 100 }, BackgroundFlag::Set)
                } else {
                    console.set_char_background(x, y, colors::Color { r: 50, g: 50, b: 150 }, BackgroundFlag::Set)
                }
            }
        }
    }

    fn clear(&self, console: &mut Console) {}
}