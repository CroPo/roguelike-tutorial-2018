use map_objects::map::GameMap;
use tcod::map::Map;
use settings::Settings;
use ecs::Ecs;
use ecs::component::Position;

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

pub fn recompute_fov(ecs: &Ecs, fov_map: &mut Map, settings: &Settings) {
    let p = ecs.get_component::<Position>(ecs.player_entity_id).unwrap();
    fov_map.compute_fov(p.position.0, p.position.1,
                        settings.fov_radius(),
                        settings.fov_light_walls(),
                        settings.fov_algorithm());
}