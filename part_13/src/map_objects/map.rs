use std::cmp;

use rand::prelude::*;

use tcod::Console;
use tcod::BackgroundFlag;
use tcod::Map;
use tcod::colors;

use json::JsonValue;

use map_objects::tile::Tile;
use map_objects::rectangle::Rect;

use map_objects::color::Color;

use ecs::Ecs;
use ecs::creature::CreatureTemplate;
use ecs::component::Position;
use ecs::item::ItemTemplate;
use settings::Settings;
use savegame::{Serialize, Deserialize};
use ecs::component::Stairs;
use ecs::component::Render;
use render::RenderOrder;
use ecs::component::Name;
use random_utils::by_dungeon_level;
use std::borrow::Cow;

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

    fn reset_tiles(&mut self) {
        let size = self.tiles.len();
        self.tiles = vec![Tile::new(true, true); size]
    }

    pub fn is_move_blocked(&self, x: i32, y: i32) -> bool {
        if self.get_tile(x as usize, y as usize).block_move {
            return true;
        }
        false
    }

    /// Try to create a new dungeon map and place entities.
    /// Returns true if successful, false if failed
    pub fn make_map(&mut self,
                    ecs: &mut Ecs, settings: &Settings, floor_number: u8) -> bool{

        self.reset_tiles();
        let mut rooms: Vec<Rect> = Vec::new();
        let mut rng = thread_rng();

        let mut failed_attempts = 0;
        let mut failed_attempts_room = 0;

        'roomloop: loop {

            if (rooms.len() as i32) == settings.max_rooms() {
                break 'roomloop
            }

            if (rooms.len() as i32) >= settings.min_rooms() && failed_attempts_room == settings.max_attempts_room() {
                break 'roomloop
            }

            if (rooms.len() as i32) < settings.min_rooms() && failed_attempts == settings.max_attempts_min_rooms() {
                return false
            }


            let w = rng.gen_range(settings.room_min_size(), settings.room_max_size());
            let h = rng.gen_range(settings.room_min_size(), settings.room_max_size());

            let x = rng.gen_range(0, self.dimensions.0 - w - 1);
            let y = rng.gen_range(0, self.dimensions.1 - h - 1);

            let new_room = Rect::new(x, y, w, h);

            for other_room in &rooms {
                if new_room.intersect(&other_room) {
                    failed_attempts+=1;
                    failed_attempts_room+=1;
                    continue 'roomloop;
                }
            }

            failed_attempts_room = 0;

            self.create_room(&new_room);
            let center = new_room.center();

            if rooms.len() == 0 {
                self.create_or_update_player(ecs, center);
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
            self.place_entities(&new_room, ecs,
                                settings.max_monsters_per_room(),
                                settings.max_items_per_room(), floor_number);
            rooms.push(new_room);
        }

        self.add_stair(ecs, &rooms[rooms.len()-1]);
        true
    }

    fn create_or_update_player(&self, ecs: &mut Ecs, position: (i32, i32)) {
        let id = ecs.player_entity_id;
        if ecs.player_entity_id > 0 {
            let p = ecs.get_component_mut::<Position>(id).unwrap();
            p.move_absolute(position);
        } else {
            CreatureTemplate::Player.create_on_position(ecs, &self, position);
        }
    }

    fn add_stair(&mut self, ecs: &mut Ecs, room: &Rect) {
        let id = ecs.create_entity();
        ecs.register_component(id, Stairs {});
        ecs.register_component(id, Position {
            entity_id: id,
            position: room.center(),
            is_blocking: false,
        });
        ecs.register_component(id, Render::new(id, '>',
                                               colors::WHITE, RenderOrder::Stair));
        ecs.register_component(id, Name {
            name: String::from("Stairs"),
        });
    }

    fn create_room(&mut self, room: &Rect) {
        for x in room.tl.0 + 1..room.lr.0 {
            for y in room.tl.1 + 1..room.lr.1 {
                self.get_tile_mut(x as usize, y as usize).block_move = false;
                self.get_tile_mut(x as usize, y as usize).block_sight = false;
            }
        }
    }

    fn place_entities(&mut self, room: &Rect, ecs: &mut Ecs,
                        max_monsters_per_room: Cow<Vec<(i32, i32)>>, max_items_per_room: Cow<Vec<(i32, i32)>>,
                      floor_number: u8 ) {
        let mut rng = thread_rng();

        let monster_count = rng.gen_range(0, by_dungeon_level(max_monsters_per_room, floor_number));
        let item_count = rng.gen_range(0, by_dungeon_level(max_items_per_room, floor_number));

        for _ in 0..monster_count {
            let x = rng.gen_range(room.tl.0 + 1, room.lr.0 - 1);
            let y = rng.gen_range(room.tl.1 + 1, room.lr.1 - 1);

            if !ecs.get_all::<Position>().iter().any(|(_, p)| p.position.0 == x && p.position.1 == y) {
                CreatureTemplate::create_random(ecs, &self, (x, y), floor_number);
            }
        }

        for _ in 0..item_count {
            let x = rng.gen_range(room.tl.0 + 1, room.lr.0 - 1);
            let y = rng.gen_range(room.tl.1 + 1, room.lr.1 - 1);

            if !ecs.get_all::<Position>().iter().any(|(_, p)| p.position.0 == x && p.position.1 == y) {
                ItemTemplate::create_random(ecs, (x,y), floor_number);
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

impl Serialize for GameMap {
    fn serialize(&self) -> JsonValue {

        let mut tiles = JsonValue::new_array();

        for tile in self.tiles.iter() {
            tiles.push(tile.serialize());
        }

        object!(
            "width" => self.dimensions.0,
            "height" => self.dimensions.1,
            "tiles" => tiles
        )
    }
}

impl Deserialize for GameMap {
    fn deserialize(json: &JsonValue) -> Self {

        let mut tiles = vec!();

        for t in json["tiles"].members() {
            tiles.push(Tile::deserialize(t));
        }

        GameMap {
            tiles,
            dimensions: (json["width"].as_i32().unwrap(), json["height"].as_i32().unwrap()),
        }
    }
}