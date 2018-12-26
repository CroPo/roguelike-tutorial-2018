use tcod::colors;

pub enum Color {
    DarkWall,
    DarkFloor,
    LightWall,
    LightFloor,
}

impl Color {
    pub fn value(&self) -> colors::Color {

        match *self {
            Color::DarkFloor => colors::Color { r: 50, g: 50, b: 150 },
            Color::DarkWall => colors::Color { r: 0, g: 0, b: 100 },
            Color::LightFloor => colors::Color { r: 200, g: 180, b: 150 },
            Color::LightWall => colors::Color { r: 130, g: 110, b: 50 },
        }
    }
}