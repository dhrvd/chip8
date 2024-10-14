use sdl2::keyboard::Scancode;
use std::{ops::Index, ops::IndexMut};

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

    pub fn update(&mut self, scancode: Scancode, down: bool) {
        let key = match scancode {
            Scancode::X => 0x00,
            Scancode::Num1 => 0x01,
            Scancode::Num2 => 0x02,
            Scancode::Num3 => 0x03,
            Scancode::Q => 0x04,
            Scancode::W => 0x05,
            Scancode::E => 0x06,
            Scancode::A => 0x07,
            Scancode::S => 0x08,
            Scancode::D => 0x09,
            Scancode::Z => 0x0A,
            Scancode::C => 0x0B,
            Scancode::Num4 => 0x0C,
            Scancode::R => 0x0D,
            Scancode::F => 0x0E,
            Scancode::V => 0x0F,
            _ => return,
        };

        self[key] = down;
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

impl IndexMut<u8> for Keypad {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.keys[index as usize]
    }
}
