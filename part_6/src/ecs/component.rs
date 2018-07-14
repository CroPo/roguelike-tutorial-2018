use std::any::Any;

/// Used to indentify an Component
pub trait Component: Any {}

/// A Component which contains informations of an `Entity`s position on the Map, and methods to
/// interact with it
pub struct Position {
    pub position: (i32, i32),
    pub is_blocking: bool,
}

impl Position {

    pub fn new(is_blocking : bool) -> Position {
        Position {
            position: (0, 0),
            is_blocking
        }
    }

    /// Change the Position of the Entity
    pub fn mv(&mut self, vel: (i32, i32)) {
        self.position.0 += vel.0;
        self.position.1 += vel.1;
    }
}

impl Component for Position {}

