use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use ecs::Entity;
use ecs::Ecs;
use ecs::component::Position;

pub trait Render {
    fn draw(&self, console: &mut Console, ecs: &Ecs);
    fn clear(&self, console: &mut Console, ecs: &Ecs);
}

pub fn render_all(ecs: &Ecs, map: &mut GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Offscreen, root_console: &mut Root) {
    map.draw(console, fov_map, fov_recompute);

    ecs.get_all::<Position>().iter().filter(|(_, p)| {
        fov_map.is_in_fov(p.position.0, p.position.1)
    }).for_each(|(e, p)| {
        println!("Draw {} (but nor really)", e);
    });


    blit(console, (0, 0),
         (console.width(), console.height()),
         root_console, (0, 0),
         1.0, 1.0)
}

pub fn clear_all(entity_manager: &Ecs, console: &mut Console) {
    //for entity in entity_manager.entities.iter() {
    //    entity.1.clear(console);
   // }
}
