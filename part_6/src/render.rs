use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use entities::Entity;

pub trait Render {
    fn draw(&self, console: &mut Console);
    fn clear(&self, console: &mut Console);
}

pub fn render_all(objs: &Vec<Entity>, map: &mut GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Offscreen, root_console: &mut Root) {
    map.draw(console, fov_map, fov_recompute);

    for obj in objs.iter().filter(|e| fov_map.is_in_fov(e.pos.0, e.pos.1) ) {
        obj.draw(console);
    }

    blit(console, (0, 0),
         (console.width(), console.height()),
         root_console, (0, 0),
         1.0, 1.0)
}

pub fn clear_all<T: Render>(objs: &Vec<T>, console: &mut Console) {
    for obj in objs {
        obj.clear(console);
    }
}
