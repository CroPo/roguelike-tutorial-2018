use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use ecs::Entity;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Render;

/// Render all `Entity`s which got both the `Render` and the `Position` component assigned onto the console
pub fn render_all(ecs: &Ecs, map: &mut GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Offscreen, root_console: &mut Root) {
    map.draw(console, fov_map, fov_recompute);

    ecs.get_all::<Position>().iter().filter(|(_, p)| {
        fov_map.is_in_fov(p.position.0, p.position.1)
    }).for_each(|(e, p)| {
        let render_component = ecs.get_component::<Render>(*e);
        match render_component {
            Some(r) => {
                r.draw(console, p.position)
            },
            None => ()
        }
    });


    blit(console, (0, 0),
         (console.width(), console.height()),
         root_console, (0, 0),
         1.0, 1.0)
}


/// Clear all `Entity`s which got both the `Render` and the `Position` component assigned from the console
pub fn clear_all(ecs: &Ecs, console: &mut Console) {
    ecs.get_all::<Position>().iter().for_each(|(e, p)| {
        let render_component = ecs.get_component::<Render>(*e);
        match render_component {
            Some(r) => {
                r.clear(console, p.position)
            },
            None => ()
        }
    });
}
