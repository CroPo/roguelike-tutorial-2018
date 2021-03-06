use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::cell::RefMut;

use json::{JsonValue};
use json;

use game::Game;
use settings::Settings;

const SAVE_FILE_NAME: &str = "savegame.dat";

pub fn save(game: &RefMut<Game>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(SAVE_FILE_NAME).unwrap();


    let data = game.serialize().to_string();
    file.write_all(data.into_bytes().as_slice());
}

pub fn load() -> Option<JsonValue> {

    match OpenOptions::new().read(true).open(SAVE_FILE_NAME) {
        Ok(mut f) => deserialize( &mut f),
        _ => None
    }

}

fn deserialize(file: &mut File) -> Option<JsonValue> {
    let mut data = String::new();
    file.read_to_string(&mut data);

    match json::parse(&data) {
        Ok(parsed) => Some(parsed),
        Err(e) => None
    }
}


pub fn delete() {
    fs::remove_file(SAVE_FILE_NAME);
}


pub trait Serialize {
    fn serialize(&self) -> JsonValue;
}

pub trait Deserialize {
    fn deserialize(json: &JsonValue) -> Self;
}