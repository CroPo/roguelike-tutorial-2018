use map_objects::map::GameMap;
use tcod::map::FovAlgorithm;
use tcod::map::Map;
use settings::Settings;

pub fn initialize_fov(game_map: &GameMap) -> Map {
    let mut fov_map = Map::new(game_map.dimensions.0, game_map.dimensions.1);

    for x in 0..game_map.dimensions.0 {
        for y in 0..game_map.dimensions.1 {
            let tile = game_map.get_tile(x as usize, y as usize);
            fov_map.set(x, y, !tile.block_sight, !tile.block_move);
        }
    }
    fov_map
}

pub fn recompute_fov(fov_map: &mut Map, position: (i32, i32), settings: &Settings) {
    fov_map.compute_fov(position.0, position.1,
                        settings.fov_radius(),
                        settings.fov_light_walls(),
                        settings.fov_algorithm());

}