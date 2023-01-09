use tao::keyboard::KeyCode;

pub fn is_relevant_key_code(key_code: KeyCode) -> bool {
    matches!(
        key_code,
        KeyCode::Digit0
            | KeyCode::Digit1
            | KeyCode::Digit2
            | KeyCode::Digit3
            | KeyCode::Digit4
            | KeyCode::Digit5
            | KeyCode::Digit6
            | KeyCode::Digit7
            | KeyCode::Digit8
            | KeyCode::Digit9
            | KeyCode::Numpad0
            | KeyCode::Numpad1
            | KeyCode::Numpad2
            | KeyCode::Numpad3
            | KeyCode::Numpad4
            | KeyCode::Numpad5
            | KeyCode::Numpad6
            | KeyCode::Numpad7
            | KeyCode::Numpad8
            | KeyCode::Numpad9
            | KeyCode::KeyA
            | KeyCode::KeyB
            | KeyCode::KeyC
            | KeyCode::KeyD
            | KeyCode::KeyE
            | KeyCode::KeyF
    )
}

pub fn key_code_to_u8(key_code: KeyCode) -> u8 {
    match key_code {
        KeyCode::Digit0 | KeyCode::Numpad0 => 0,
        KeyCode::Digit1 | KeyCode::Numpad1 => 1,
        KeyCode::Digit2 | KeyCode::Numpad2 => 2,
        KeyCode::Digit3 | KeyCode::Numpad3 => 3,
        KeyCode::Digit4 | KeyCode::Numpad4 => 4,
        KeyCode::Digit5 | KeyCode::Numpad5 => 5,
        KeyCode::Digit6 | KeyCode::Numpad6 => 6,
        KeyCode::Digit7 | KeyCode::Numpad7 => 7,
        KeyCode::Digit8 | KeyCode::Numpad8 => 8,
        KeyCode::Digit9 | KeyCode::Numpad9 => 9,
        KeyCode::KeyA => 0x0A,
        KeyCode::KeyB => 0x0B,
        KeyCode::KeyC => 0x0C,
        KeyCode::KeyD => 0x0D,
        KeyCode::KeyE => 0x0E,
        KeyCode::KeyF => 0x0F,
        _ => panic!("Cannot match key code {:?} to u8", key_code),
    }
}
