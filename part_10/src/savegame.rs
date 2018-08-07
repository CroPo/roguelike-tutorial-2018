use game::Game;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use std::fs;

use json::JsonValue;

const SAVE_FILE_NAME: &str = "savegame.dat";

pub fn write(game: &Serialize) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(SAVE_FILE_NAME).unwrap();


    let data = game.serialize().to_string();
    file.write_all(data.into_bytes().as_slice());
}

pub fn delete() {
    fs::remove_file(SAVE_FILE_NAME);
}


pub trait Serialize {
    fn serialize(&self) -> JsonValue;
}