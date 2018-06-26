use tcod::colors;
use tcod::Console;
use tcod::BackgroundFlag;

use map_objects::tile::Tile;
use render::Render;
use map_objects::rectangle::Rect;

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
        let mut tiles = vec![Tile::new(true, true); height * width];

        tiles
    }

    pub fn is_move_blocked(&self, x : i32, y:i32) -> bool {
        if self.get_tile(x as usize, y as usize).block_move {
            return true;
        }
        false
    }

    pub fn make_map(&mut self) {
        self.create_room(&Rect::new(20,15,10,15));
        self.create_room(&Rect::new(35,15,10,15));
    }

    fn create_room(&mut self, room: &Rect) {
        for x in room.tl.0+1..room.lr.0 {
            for y in  room.tl.1+1..room.lr.1 {
                self.get_tile_mut(x as usize,y as usize).block_move = false;
                self.get_tile_mut(x as usize,y as usize).block_sight = false;
            }
        }
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

    fn clear(&self, _console: &mut Console) {}
}