use tcod::colors;
use std::collections::HashMap;


pub enum Colors {
    DarkWall,
    DarkFloor,
    LightWall,
    LightFloor,
}

impl Colors {
    pub fn value(&self) -> colors::Color {

        match *self {
            Colors::DarkFloor => colors::Color { r: 50, g: 50, b: 150 },
            Colors::DarkWall => colors::Color { r: 0, g: 0, b: 100 },
            Colors::LightFloor => colors::Color { r: 200, g: 180, b: 150 },
            Colors::LightWall => colors::Color { r: 130, g: 110, b: 50 },
        }
    }
}