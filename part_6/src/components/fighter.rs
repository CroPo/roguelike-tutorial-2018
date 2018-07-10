pub struct Fighter {
    pub max_hp : i32,
    pub hp : i32,
    pub defense : i32,
    pub attack : i32,
}

impl Fighter {
    pub fn new(hp: i32, defense: i32, power: i32) -> Fighter {
        Fighter {
            max_hp : hp, hp, defense, attack : power
        }
    }
}