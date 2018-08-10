use std::fmt::{ Display, Formatter, Result, Debug };
use std::ops::DerefMut;
use std::cell::RefMut;

use tcod::console::{Console, Root, blit, Offscreen};
use tcod::image;

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
use game::state::GameState;
use json::JsonValue;
use savegame::Deserialize;

use game::Game;
use engine::Engine;
use tcod::image::Image;
use ecs::component::Stairs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RenderOrder {
    Stair = 1,
    Corpse = 2,
    Item = 3,
    Actor = 4,
}

impl Display for RenderOrder {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

impl Deserialize for RenderOrder {
    fn deserialize(json: &JsonValue) -> Self {
        match json.as_str() {
            Some("Corpse") => RenderOrder::Corpse,
            Some("Item") => RenderOrder::Item,
            Some("Actor") => RenderOrder::Actor,
            Some("Stair") => RenderOrder::Stair,
            _ => RenderOrder::Corpse
        }
    }
}


/// Render all `Entity`s which got both the `Render` and the `Position` component assigned onto the console
pub fn render_all(engine: &Engine, game: &RefMut<Game>) {

    match engine.state {
        GameState::MainMenu => render_main_menu(&engine),
        _ => render_game(&engine, &game)
    }
}

fn render_game(engine: &Engine, game: &RefMut<Game>) {
    let ecs = game.ecs.borrow();
    let mut map = game.map.borrow_mut();
    let fov_map = game.fov_map.borrow();

    let mut root_console = engine.root_console.borrow_mut();

    let mut console = Offscreen::new(engine.settings.screen_width(), engine.settings.screen_height());
    let mut panel = Offscreen::new(engine.settings.screen_width(), engine.settings.panel_height());

    map.draw(&mut console, &fov_map);

    let component_ids = ecs.get_all_ids::<Render>();
    let mut ids_filtered: Vec<&EntityId> = component_ids.iter().filter(|id| {
        if let Some(p) = ecs.get_component::<Position>(**id) {
            fov_map.is_in_fov(p.position.0, p.position.1)
                || ( map.get_tile(p.position.0 as usize, p.position.1 as usize).explored &&
                ecs.has_component::<Stairs>(**id) )
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
        c.draw(&ecs, &mut console)
    });


    blit(&console, (0, 0),
         (console.width(), console.height()),
         root_console.deref_mut(), (0, 0),
         1.0, 1.0);


    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.set_default_background(colors::BLACK);
    panel.clear();

    panel.print_ex(1, 0, BackgroundFlag::None, TextAlignment::Left,
                   get_names_under_mouse(&ecs, &fov_map, engine.mouse_pos));

    if let Some(p) = ecs.get_component::<Actor>(ecs.player_entity_id) {
        panel.set_default_background(colors::BLACK);
        render_bar(&mut panel, (1, 1), engine.settings.bar_width(),
                   "HP", p.hp, p.max_hp,
                   colors::RED, colors::DARK_RED);
    }

    panel.print_ex(1, 3, BackgroundFlag::None, TextAlignment::Left, format!("Dungeon level: {}", game.floor_number));

    game.log_panel.render(&mut panel);

    blit(&panel, (0, 0),
         (panel.width(), panel.height()),
         root_console.deref_mut(), engine.settings.panel_pos(),
         1.0, 1.0);


    match engine.state {
        GameState::ShowInventoryUse => inventory_menu(root_console.deref_mut(), &ecs, "Press the key next to an item to use it, or Esc to cancel.",
                                                      50, console.width(), console.height()),
        GameState::ShowInventoryDrop => inventory_menu(root_console.deref_mut(), &ecs, "Press the key next to an item to drop it, or Esc to cancel.",
                                                       50, console.width(), console.height()),
        GameState::ShowQuitGameMenu => selection_menu(root_console.deref_mut(), "",
                                                      vec![String::from("Save & Quit"), String::from("Cancel")],
                                                      24, console.width(), console.height()),
        GameState::PlayerDead => message_box(root_console.deref_mut(), "YOU ARE DEAD. Press Escape to return to the main menu",
                                             console.width(), console.height()),
        _ => ()
    }
    root_console.flush()
}

fn render_main_menu(engine: &Engine) {
    let mut root_console = engine.root_console.borrow_mut();

    let background = Image::from_file("menu_background1.png").unwrap();

    image::blit_2x(&background, (0,0),
                   (-1, -1),
                   root_console.deref_mut(), (0,0));

    root_console.set_default_foreground(colors::LIGHT_YELLOW);

    root_console.print_ex(engine.settings.screen_width()/2, engine.settings.screen_height() /2 - 4,
                          BackgroundFlag::None, TextAlignment::Center,
                    "/r/roguelikedev Tutorial Series 2018");

    root_console.print_ex(engine.settings.screen_width()/2, engine.settings.screen_height() - 2,
                          BackgroundFlag::None, TextAlignment::Center,
                          "by /u/CrocodileSpacePope");

    selection_menu(&mut root_console, "",
                   vec![String::from("New game"), String::from("Continue last game"), String::from("Quit")],
                   24, engine.settings.screen_width(), engine.settings.screen_height());


    root_console.flush();
}


/// Render a bar to graphically represent a value
pub fn render_bar(panel: &mut Offscreen, pos: (i32, i32), width: i32, name: &str, value: u32, max: u32, bar_color: Color, back_color: Color) {
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

fn message_box(console: &mut Root, title: &str, screen_width: i32, screen_height: i32) {
    selection_menu(console, title, vec![], 24, screen_width, screen_height);
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
                panel.print_ex(self.pos.0, self.pos.1 + total_lines,
                               BackgroundFlag::None, TextAlignment::Left, l.to_string());
                total_lines += 1;
                if self.pos.1 + total_lines > self.dimensions.1 {
                    break 'l;
                }
            }
        };
    }
}
