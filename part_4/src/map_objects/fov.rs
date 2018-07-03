use map_objects::map::GameMap;
use tcod;

pub fn initialize_fov(game_map: &GameMap) -> tcod::map::Map {
    let mut fov_map = tcod::map::Map::new(game_map.dimensions.0, game_map.dimensions.1);

    for x  in 0..game_map.dimensions.0 {
        for y in 0..game_map.dimensions.1 {
            let tile = game_map.get_tile(x as usize, y as usize);
            fov_map.set(x, y, !tile.block_sight, !tile.block_move);
        }
    }
    fov_map
}