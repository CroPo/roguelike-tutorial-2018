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

use tcod::console::{Root, Offscreen};
use tcod::FontLayout;
use tcod::FontType;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::FovAlgorithm;

use map_objects::map::GameMap;
use map_objects::fov;

use ecs::Ecs;
use ecs::component::Position;
use ecs::component::MonsterAi;
use ecs::action::EntityAction;
use ecs::component::Actor;
use ecs::component::Corpse;
use message::MessageLog;
use render::MessagePanel;
use std::rc::Rc;
use tcod::input::Event;
use tcod::input::Mouse;
use ecs::id::EntityId;
use ecs::component::Item;
use render::render_all;
use render::clear_all;
use settings::Settings;
use game::state::GameState;
use game::Game;


fn main() {
    let settings = Settings::new();
    let mut root = Root::initializer()
        .size(settings.screen_width(), settings.screen_height())
        .title(settings.title())
        .font(settings.font_path(), settings.font_layout())
        .font_type(settings.font_type())
        .init();

    let mut game = Game::new(&settings);
    game.run(&mut root);
}

