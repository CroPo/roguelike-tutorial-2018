use std::cmp;

use rand::prelude::*;

use tcod::Console;
use tcod::BackgroundFlag;
use tcod::Map;

use map_objects::tile::Tile;
use map_objects::rectangle::Rect;

use map_objects::color::Color;

use ecs::Ecs;
use ecs::creature::CreatureTemplate;
use ecs::component::Position;
use ecs::item::ItemTemplate;

pub struct GameMap {
    pub dimensions: (i32, i32),
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
        vec![Tile::new(true, true); height * width]
    }

    pub fn is_move_blocked(&self, x: i32, y: i32) -> bool {
        if self.get_tile(x as usize, y as usize).block_move {
            return true;
        }
        false
    }

    pub fn make_map(&mut self,
                    max_rooms: i32,
                    room_min_size: i32, room_max_size: i32,
                    ecs: &mut Ecs,
                    max_monsters_per_room: i32, max_items_per_room: i32) {
        let mut rooms: Vec<Rect> = Vec::new();
        let mut rng = thread_rng();

        'roomloop: while max_rooms > rooms.len() as i32 {
            let w = rng.gen_range(room_min_size, room_max_size);
            let h = rng.gen_range(room_min_size, room_max_size);

            let x = rng.gen_range(0, self.dimensions.0 - w - 1);
            let y = rng.gen_range(0, self.dimensions.1 - h - 1);

            let new_room = Rect::new(x, y, w, h);

            for other_room in &rooms {
                if new_room.intersect(&other_room) {
                    continue 'roomloop;
                }
            }

            self.create_room(&new_room);
            let center = new_room.center();

            if rooms.len() == 0 {
                CreatureTemplate::Player.create_on_position(ecs, center);
            } else {
                let prev_center = rooms[rooms.len() - 1].center();

                if rng.gen() {
                    self.create_h_tunnel(prev_center.0, center.0, prev_center.1);
                    self.create_v_tunnel(prev_center.1, center.1, center.0);
                } else {
                    self.create_v_tunnel(prev_center.1, center.1, prev_center.0);
                    self.create_h_tunnel(prev_center.0, center.0, center.1);
                }
            }
            self.place_entities(&new_room, ecs, max_monsters_per_room, max_items_per_room);
            rooms.push(new_room);
        }
    }

    fn create_room(&mut self, room: &Rect) {
        for x in room.tl.0 + 1..room.lr.0 {
            for y in room.tl.1 + 1..room.lr.1 {
                self.get_tile_mut(x as usize, y as usize).block_move = false;
                self.get_tile_mut(x as usize, y as usize).block_sight = false;
            }
        }
    }

    fn place_entities(&mut self, room: &Rect, ecs: &mut Ecs, max_monsters_per_room: i32, max_items_per_room :i32) {
        let mut rng = thread_rng();

        let monster_count = rng.gen_range(0, max_monsters_per_room);
        let item_count = rng.gen_range(0, max_items_per_room);

        for _ in 0..monster_count {
            let x = rng.gen_range(room.tl.0 + 1, room.lr.0 - 1);
            let y = rng.gen_range(room.tl.1 + 1, room.lr.1 - 1);

            if !ecs.get_all::<Position>().iter().any(|(_, p)| p.position.0 == x && p.position.1 == y) {
                let mut monster = if rng.gen_range(0, 100) < 80 {
                    CreatureTemplate::Troll
                } else {
                    CreatureTemplate::Orc
                };

                monster.create_on_position(ecs, (x, y));
            }
        }

        for _ in 0..item_count {
            let x = rng.gen_range(room.tl.0 + 1, room.lr.0 - 1);
            let y = rng.gen_range(room.tl.1 + 1, room.lr.1 - 1);

            if !ecs.get_all::<Position>().iter().any(|(_, p)| p.position.0 == x && p.position.1 == y) {
                let item = ItemTemplate::HealthPotion;
                item.create_on_position(ecs, (x, y));
            }
        }

    }

    fn create_h_tunnel(&mut self, x_start: i32, x_end: i32, y: i32) {
        for x in cmp::min(x_start, x_end)..cmp::max(x_start, x_end) + 1 {
            self.get_tile_mut(x as usize, y as usize).block_move = false;
            self.get_tile_mut(x as usize, y as usize).block_sight = false;
        }
    }

    fn create_v_tunnel(&mut self, y_start: i32, y_end: i32, x: i32) {
        for y in cmp::min(y_start, y_end)..cmp::max(y_start, y_end) + 1 {
            self.get_tile_mut(x as usize, y as usize).block_move = false;
            self.get_tile_mut(x as usize, y as usize).block_sight = false;
        }
    }

    pub fn draw(&mut self, console: &mut Console, fov_map: &Map) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let tile = self.get_tile_mut(x as usize, y as usize);

                let wall = tile.block_move;
                let visible = fov_map.is_in_fov(x, y);

                if visible {
                    if wall {
                        console.set_char_background(x, y, Color::LightWall.value(), BackgroundFlag::Set)
                    } else {
                        console.set_char_background(x, y, Color::LightFloor.value(), BackgroundFlag::Set)
                    }
                    tile.explored = true;
                } else if tile.explored {
                    if wall {
                        console.set_char_background(x, y, Color::DarkWall.value(), BackgroundFlag::Set)
                    } else {
                        console.set_char_background(x, y, Color::DarkFloor.value(), BackgroundFlag::Set)
                    }
                }
            }
        }
    }
}