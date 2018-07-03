use tcod::console::{Console, Root};

use map_objects::map::GameMap;
use tcod::Map;

pub trait Render {
    fn draw(&self, console: &mut Console);
    fn clear(&self, console: &mut Console);
}

pub fn render_all<T: Render>(objs: &Vec<T>, map: &GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Root) {

    map.draw(console, fov_map, fov_recompute);

    for obj in objs {
        obj.draw(console);
    }
}

pub fn clear_all<T: Render>(objs: &Vec<T>, console: &mut Console) {
    for obj in objs {
        obj.clear(console);
    }
}
