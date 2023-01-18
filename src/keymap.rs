use core::panic;
use std::{collections::HashMap, fs::OpenOptions, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use tao::keyboard::KeyCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap {
    pub keys: HashMap<KeyCode, u8>,
}

fn parse_string(data: &str) -> Result<Keymap> {
    serde_json::from_str(&data)
}

pub fn read_keymap(path: PathBuf) -> Result<Keymap> {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut data = Vec::new();
    if let Err(err) = file.read_to_end(&mut data) {
        panic!("Could not read file: {:?}", err);
    }

    let data = std::str::from_utf8(&data).unwrap();

    parse_string(data)
}

pub fn default_keymap() -> Keymap {
    let keys: HashMap<KeyCode, u8> = HashMap::from([
        (KeyCode::Digit1, 1),
        (KeyCode::Digit2, 2),
        (KeyCode::Digit3, 3),
        (KeyCode::Digit4, 12),
        (KeyCode::KeyQ, 4),
        (KeyCode::KeyW, 5),
        (KeyCode::KeyE, 6),
        (KeyCode::KeyR, 13),
        (KeyCode::KeyA, 7),
        (KeyCode::KeyS, 8),
        (KeyCode::KeyD, 9),
        (KeyCode::KeyF, 14),
        (KeyCode::KeyZ, 10),
        (KeyCode::KeyX, 0),
        (KeyCode::KeyC, 11),
        (KeyCode::KeyV, 15),
    ]);
    Keymap { keys }
}
