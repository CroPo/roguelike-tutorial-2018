use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use ecs::Entity;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Render;
use ecs::id::EntityId;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderOrder {
    Corpse = 1,
    Item = 2,
    Actor = 3,
}

/// Render all `Entity`s which got both the `Render` and the `Position` component assigned onto the console
pub fn render_all(ecs: &Ecs, map: &mut GameMap, fov_map: &Map, fov_recompute: bool, console: &mut Offscreen, root_console: &mut Root) {
    map.draw(console, fov_map, fov_recompute);


    let component_ids = ecs.get_all_ids::<Render>();
    let mut ids_filtered: Vec<&EntityId> = component_ids.iter().filter(|id| {
        if let Some(p) = ecs.get_component::<Position>(**id) {
            fov_map.is_in_fov(p.position.0, p.position.1)
        } else {
            false
        }
    }).collect();
    ids_filtered.sort_by(|id_a, id_b|{

        let comp_a = ecs.get_component::<Render>(**id_a).unwrap();
        let comp_b = ecs.get_component::<Render>(**id_b).unwrap();

        comp_a.order.cmp(&comp_b.order)
    });
    ids_filtered.iter().for_each(|id| {
        let c = ecs.get_component::<Render>(**id).unwrap();
        c.draw(ecs, console)
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
                r.clear(ecs, console)
            }
            None => ()
        }
    });
}
