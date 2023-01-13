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
    let data = "{\"keys\":{\"Digit0\":0,\"Numpad0\":0,\"Digit1\":1,\"Numpad1\":1,\"Digit2\":2,\"Numpad2\":2,\"Digit3\":3,\"Numpad3\":3,\"Digit4\":4,\"Numpad4\":4,\"Digit5\":5,\"Numpad5\":5,\"Digit6\":6,\"Numpad6\":6,\"Digit7\":7,\"Numpad7\":7,\"Digit8\":8,\"Numpad8\":8,\"Digit9\":9,\"Numpad9\":9,\"KeyA\":10,\"KeyB\":11,\"KeyC\":12,\"KeyD\":13,\"KeyE\":14,\"KeyF\":15}}";
    parse_string(data).unwrap()
}
