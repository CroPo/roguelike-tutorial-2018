use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use entities::Entity;
use entities::EntityManager;

pub trait Render {
    fn draw(&self, console: &mut Console);
    fn clear(&self, console: &mut Console);
}

pub fn render_all(entity_manager: &EntityManager, map: &mut GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Offscreen, root_console: &mut Root) {
    map.draw(console, fov_map, fov_recompute);

    for entity in entity_manager.entities.iter().filter(|e| fov_map.is_in_fov(e.1.pos.0, e.1.pos.1) ) {
        entity.1.draw(console);
    }

    blit(console, (0, 0),
         (console.width(), console.height()),
         root_console, (0, 0),
         1.0, 1.0)
}

pub fn clear_all(entity_manager: &EntityManager, console: &mut Console) {
    for entity in entity_manager.entities.iter() {
        entity.1.clear(console);
    }
}
