extern crate tcod;
extern crate rand;
extern crate textwrap;

#[macro_use]
extern crate json;

mod ecs;
mod render;
mod map_objects;
mod game;
mod message;
mod settings;
mod savegame;
mod engine;
mod random_utils;

use engine::Engine;
use settings::Settings;

fn main() {
    Engine::run(&Settings::new());
}

