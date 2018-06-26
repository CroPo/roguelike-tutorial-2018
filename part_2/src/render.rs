use tcod::console::{Offscreen, Console, Root};
use tcod::console;
use tcod::FontLayout;
use tcod::FontType;

use std::borrow::BorrowMut;

pub trait Render {
    fn draw(&self, console: &mut Console);
    fn clear(&self, console: &mut Console);
}


pub fn render_all<T: Render, U: Render>(objs: &Vec<T>, map: &U, console: &mut Root, screen_width: i32, screen_height: i32) {

    let mut offscreen = Box::new(Offscreen::new(screen_width, screen_height));

    map.draw(&mut offscreen);

    for obj in objs {
        obj.draw(&mut offscreen);
    }

    console::blit(&offscreen,
                  (0, 0),
                  (screen_width, screen_height),
                  console,
                  (0, 0), 1.0, 1.0);
}

pub fn clear_all<T: Render>(objs: &Vec<T>,console: &mut Console) {
    for obj in objs {
        obj.clear(console);
    }
}
