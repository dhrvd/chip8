use macroquad::input::{is_key_down, KeyCode};
use std::ops::Index;

const KEY_MAP: [(KeyCode, u8); 16] = [
    (KeyCode::X, 0x00),
    (KeyCode::Key1, 0x01),
    (KeyCode::Key2, 0x02),
    (KeyCode::Key3, 0x03),
    (KeyCode::Q, 0x04),
    (KeyCode::W, 0x05),
    (KeyCode::E, 0x06),
    (KeyCode::A, 0x07),
    (KeyCode::S, 0x08),
    (KeyCode::D, 0x09),
    (KeyCode::Z, 0x0A),
    (KeyCode::C, 0x0B),
    (KeyCode::Key4, 0x0C),
    (KeyCode::R, 0x0D),
    (KeyCode::F, 0x0E),
    (KeyCode::V, 0x0F),
];

pub struct Keypad {
    keys: [bool; 16],
    pub waiting: bool,
    pub register: usize,
    pub down: u8,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            waiting: false,
            register: 0,
            down: 0,
        }
    }

    pub fn update(&mut self) {
        for (key_code, i) in KEY_MAP {
            self.keys[i as usize] = is_key_down(key_code);
        }
    }

    pub fn wait_for_key(&self) -> Option<u8> {
        (0..16).find(|&i| self[i])
    }
}

impl Index<u8> for Keypad {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        &self.keys[index as usize]
    }
}
