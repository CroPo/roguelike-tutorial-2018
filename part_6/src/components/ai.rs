pub trait Ai {
    fn take_turn(&self);
}

pub struct BasicMonster {

}

impl BasicMonster {
    pub fn new() -> BasicMonster {
        BasicMonster {}
    }
}

impl Ai for BasicMonster {

    fn take_turn(&self) {
        println!("some Monster takes some turn")
    }
}