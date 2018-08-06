use game::Game;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use std::fs;

const SAVE_FILE_NAME: &str = "savegame.dat";

pub fn write(game: &Game) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(SAVE_FILE_NAME).unwrap();


    //let data = to_string(game).unwrap();
    //file.write_all(data.into_bytes().as_slice());
}

pub fn delete() {
    fs::remove_file(SAVE_FILE_NAME);
}