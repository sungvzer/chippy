use std::collections::HashMap;

use tao::keyboard::KeyCode;

pub fn parse_key_code(key_code: KeyCode, key_map: &HashMap<KeyCode, u8>) -> Option<u8> {
    key_map.get(&key_code).copied()
}
