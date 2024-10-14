use rand::{self, Rng};

use crate::{
    display::{HEIGHT, WIDTH},
    emulator::Emulator,
};

impl Emulator {
    pub fn execute(&mut self, instruction: u16) {
        let i = ((instruction & 0xF000) >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match (i, x, y, n) {
            (0x00, 0x00, 0x0E, 0x00) => self.display.clear(), // 00EE: clear screen
            (0x01, _, _, _) => self.pc = nnn,                 // 1NNN: jump

            // 00EE and 2NNN: subroutines
            (0x00, 0x00, 0x0E, 0x0E) => self.pc = self.stack.pop(),
            (0x02, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }

            // 3XNN, 4XNN, 5XY0, 9XY0: skip conditionally
            (0x03, _, _, _) => {
                if self.v[x] == nn {
                    self.pc += 2
                }
            }
            (0x04, _, _, _) => {
                if self.v[x] != nn {
                    self.pc += 2
                }
            }
            (0x05, _, _, 0x00) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2
                }
            }
            (0x09, _, _, 0x00) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2
                }
            }

            (0x06, _, _, _) => self.v[x] = nn, // 6XNN: set vx to nn
            (0x07, _, _, _) => self.v[x] = self.v[x].wrapping_add(nn), // 7XNN: add nn to vx

            (0x08, _, _, 0x00) => self.v[x] = self.v[y], // 8XY0: set vx to value of vy

            // 8XY1, 8XY2, 8XY3: bitwise operations
            (0x08, _, _, 0x01) => self.v[x] |= self.v[y],
            (0x08, _, _, 0x02) => self.v[x] &= self.v[y],
            (0x08, _, _, 0x03) => self.v[x] ^= self.v[y],

            // 8XY4: Add vy to vx
            (0x08, _, _, 0x04) => {
                let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = sum;
                self.v[0x0F] = carry as u8;
            }

            // 8XY5 and 8XY7: subract
            (0x08, _, _, 0x05) => {
                let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = diff;
                self.v[0x0F] = !borrow as u8;
            }
            (0x08, _, _, 0x07) => {
                let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = diff;
                self.v[0x0F] = !borrow as u8;
            }

            // 8XY6 and 8XYE: shift
            (0x08, _, _, 0x06) => {
                self.v[0x0F] = self.v[x] & 1;
                self.v[x] >>= 1;
            }
            (0x08, _, _, 0x0E) => {
                self.v[0x0F] = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
            }

            (0x0A, _, _, _) => self.i = nnn, // ANNN: set index register to nnn
            (0x0B, _, _, _) => self.pc = self.v[0] as u16 + nnn, // BNNN: jump to v0 + nnn
            (0x0C, _, _, _) => self.v[x] = self.thread_range.gen_range(0..=255) & nn, // CXNN: random

            // DXYN: draw an n pixels tall and 8 pixels wide sprite at (vx, vy).
            (0x0D, _, _, _) => {
                let vx = self.v[x];
                let vy = self.v[y];

                self.v[0x0F] = 0;

                for row in 0..n {
                    let sprite_data = self.memory[self.i + row as u16];

                    for i in 0..8 {
                        let x_coord = (vx + i) % WIDTH;
                        let y_coord = (vy + row) % HEIGHT;

                        if (sprite_data & (0x80 >> i)) != 0 {
                            let pixel_state = self.display.get(x_coord, y_coord);
                            self.display.set(x_coord, y_coord, !pixel_state);

                            if pixel_state {
                                self.v[0x0F] = 1;
                            }
                        }
                    }
                }
            }

            // EX9E and EXA1: skip if key
            (0x0E, _, 0x09, 0x0E) => {
                if self.keypad[self.v[x]] {
                    self.pc += 2;
                }
            }
            (0x0E, _, 0x0A, 0x01) => {
                if !self.keypad[self.v[x]] {
                    self.pc += 2;
                }
            }

            // FX07, FX15 and FX18: timers
            (0x0F, _, 0x00, 0x07) => self.v[x] = self.delay_timer,
            (0x0F, _, 0x01, 0x05) => self.delay_timer = self.v[x],
            (0x0F, _, 0x01, 0x08) => self.sound_timer = self.v[x],

            // FX1E: add to vx to i
            (0x0F, _, 0x01, 0x0E) => {
                let (sum, carry) = self.i.overflowing_add(self.v[x] as u16);
                self.i = sum;
                self.v[0x0F] = carry as u8;
            }

            // FX0A: wait for key press and store to vx
            (0x0F, _, 0x00, 0x0A) => match self.keypad.wait_for_key() {
                Some(key) => {
                    self.keypad.down = key;
                    self.keypad.waiting = true;
                    self.keypad.register = x;
                }
                None => self.pc -= 2,
            },

            (0x0F, _, 0x02, 0x09) => self.i = self.v[x] as u16 * 5, // FX29: font character
            // FX33: binary-coded decimal conversion
            (0x0F, _, 0x03, 0x03) => {
                let vx = self.v[x] as f32;

                self.memory[self.i] = (vx / 100.0).floor() as u8;
                self.memory[self.i + 1] = ((vx / 10.0) % 10.0).floor() as u8;
                self.memory[self.i + 2] = (vx % 10.0) as u8;
            }

            // FX55 and FX65: store and load memory
            (0x0F, _, 0x05, 0x05) => {
                for i in 0..=x {
                    self.memory[self.i as usize + i] = self.v[i];
                }
            }
            (0x0F, _, 0x06, 0x05) => {
                for i in 0..=x {
                    self.v[i] = self.memory[self.i as usize + i];
                }
            }

            _ => unimplemented!("Unimplemented instruction: 0x{:04X}", instruction),
        }
    }
}
