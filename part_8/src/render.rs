use tcod::console::{Console, Root, blit, Offscreen};

use map_objects::map::GameMap;
use tcod::Map;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Render;
use ecs::id::EntityId;
use tcod::Color;
use tcod::colors;
use tcod::BackgroundFlag;
use tcod::TextAlignment;
use ecs::component::Actor;
use message::MessageLog;
use std::rc::Rc;
use textwrap::wrap;
use ecs::component::Name;
use ecs::component::Inventory;
use game_states::GameState;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderOrder {
    Corpse = 1,
    Item = 2,
    Actor = 3,
}

/// Render all `Entity`s which got both the `Render` and the `Position` component assigned onto the console
pub fn render_all(ecs: &Ecs, map: &mut GameMap, fov_map: &Map, game_state: &GameState,
                  console: &mut Offscreen, panel: &mut Offscreen, root_console: &mut Root,
                  bar_width: i32, panel_y: i32, log_panel: &MessagePanel, mouse_pos: (i32, i32)) {
    map.draw(console, fov_map);


    let component_ids = ecs.get_all_ids::<Render>();
    let mut ids_filtered: Vec<&EntityId> = component_ids.iter().filter(|id| {
        if let Some(p) = ecs.get_component::<Position>(**id) {
            fov_map.is_in_fov(p.position.0, p.position.1)
        } else {
            false
        }
    }).collect();
    ids_filtered.sort_by(|id_a, id_b| {
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
         1.0, 1.0);


    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.set_default_background(colors::BLACK);
    panel.clear();

    panel.print_ex(1, 0, BackgroundFlag::None, TextAlignment::Left, get_names_under_mouse(ecs, fov_map, mouse_pos));

    if let Some(p) = ecs.get_component::<Actor>(ecs.player_entity_id) {
        panel.set_default_background(colors::BLACK);
        render_bar(panel, (1, 1), bar_width, "HP", p.hp, p.max_hp, colors::RED, colors::DARK_RED);
    }
    log_panel.render(panel);

    blit(panel, (0, 0),
         (panel.width(), panel.height()),
         root_console, (0, panel_y),
         1.0, 1.0);


    if *game_state == GameState::ShowInventory || *game_state == GameState::ShowInventoryDrop {
        inventory_menu(root_console, ecs, "Inventory", 50, console.width(), console.height());
    }

    root_console.flush()
}


/// Clear all `Entity`s which got both the `Render` and the `Position` component assigned from the console
pub fn clear_all(ecs: &Ecs, console: &mut Console) {
    ecs.get_all::<Position>().iter().for_each(|(e, _)| {
        let render_component = ecs.get_component::<Render>(*e);
        match render_component {
            Some(r) => {
                r.clear(ecs, console)
            }
            None => ()
        }
    });
}

/// Render a bar to graphically represent a value
pub fn render_bar(panel: &mut Offscreen, pos: (i32, i32), width: i32, name: &str, value: i32, max: i32, bar_color: Color, back_color: Color) {
    let filled_width = (value as f64 / max as f64 * width as f64).round() as i32;

    panel.set_default_background(back_color);
    panel.rect(pos.0, pos.1, width, 1, false, BackgroundFlag::Screen);

    if filled_width > 0 {
        panel.set_default_background(bar_color);
        panel.rect(pos.0, pos.1, filled_width, 1, false, BackgroundFlag::Screen)
    }

    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(pos.0 + width / 2, pos.1, BackgroundFlag::None,
                   TextAlignment::Center, format!("{}: {}/{}", name, value, max));
}

/// Get a Vec of the names of all Entities which are under the cursor.
fn get_names_under_mouse(ecs: &Ecs, fov_map: &Map, mouse_pos: (i32, i32)) -> String {
    let mut names = vec![];

    ecs.get_all::<Position>().iter().filter(|(_, p)| {
        p.position.0 == mouse_pos.0 && p.position.1 == mouse_pos.1 && fov_map.is_in_fov(mouse_pos.0, mouse_pos.1)
    }).for_each(|(id, _)| {
        if let Some(n) = ecs.get_component::<Name>(*id) {
            names.push(n.name.clone());
        }
    });

    names.join(",")
}

/// Display a selection menu of various options
pub fn selection_menu(console: &mut Root, title: &str, options: Vec<String>, width: i32, screen_width: i32, screen_height: i32) {
    let header_height = console.get_height_rect(0, 0, width, screen_height, title);
    let height = header_height + options.len() as i32;
    let mut menu_panel = Offscreen::new(width, height);

    menu_panel.set_default_foreground(colors::WHITE);
    menu_panel.print_rect_ex(0, 0, width, height, BackgroundFlag::None, TextAlignment::Left, title);

    let mut y = header_height;
    let mut letter_index = 'a' as u8;

    for option in options {
        let text = format!("({}) {}", letter_index as char, option);
        menu_panel.print_ex(0, y, BackgroundFlag::None, TextAlignment::Left, text);
        y+=1;
        letter_index+=1;
    }

    let x = screen_width / 2 - width / 2;
    let y = screen_height / 2 - height / 2;

    blit(&menu_panel, (0, 0),
         (width, height),
         console, (x, y),
         1.0, 1.0);
}

pub fn inventory_menu(console: &mut Root, ecs: &Ecs, title: &str, width: i32, screen_width: i32, screen_height: i32) {

    if let Some(inventory) = ecs.get_component::<Inventory>(ecs.player_entity_id) {

        let items = if inventory.items.len() == 0 {
            vec!["Inventory is empty".to_string()]
        } else {
            inventory.items.iter().filter(|item_id|{
                ecs.has_component::<Name>(**item_id)
            }).map(|item_id| {
                ecs.get_component::<Name>(*item_id).unwrap().name.clone()
            }).collect()
        };

        selection_menu(console, title, items, width, screen_width, screen_height);
    }
}


pub struct MessagePanel {
    pos: (i32, i32),
    dimensions: (i32, i32),
    log: Rc<MessageLog>,
}

impl MessagePanel {
    pub fn new(pos: (i32, i32), dimensions: (i32, i32), log: Rc<MessageLog>) -> MessagePanel {
        MessagePanel {
            pos,
            dimensions,
            log,
        }
    }

    pub fn render(&self, panel: &mut Offscreen) {
        let mut total_lines = 0;

        'l: for m in self.log.messages().iter().rev() {
            let lines = wrap(&m.text, self.dimensions.0 as usize);

            panel.set_default_foreground(m.color);

            for l in lines {
                panel.print_ex(self.pos.0, self.pos.1 + total_lines, BackgroundFlag::None, TextAlignment::Left, l.to_string());
                total_lines += 1;
                if self.pos.1 + total_lines > self.dimensions.1 {
                    break 'l;
                }
            }
        };
    }
}
