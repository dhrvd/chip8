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
            (0x00, 0x00, 0x0E, 0x00) => self.cls(), // 00E0: clear screen
            (0x01, _, _, _) => self.jump(nnn),      // 1NNN: jump to adress nnn
            (0x00, 0x00, 0x0E, 0x0E) => self.ret(), // 00EE: return from subroutine
            (0x02, _, _, _) => self.call(nnn),      // 2NNN: call subroutine
            (0x06, _, _, _) => self.set_v(x, nn),   // 6XNN: set register vx to nn
            (0x07, _, _, _) => self.add(x, nn),     // 7XNN: add nn to register vx
            (0x0A, _, _, _) => self.set_i(nnn),     // ANNN: set register i to nnn
            (0x0D, _, _, _) => self.draw(x, y, n),  // DYXN: draw
            _ => unimplemented!("Unimplemented instruction: 0x{:04X}", instruction),
        }
    }

    // Clear the display, turning all pixels off to 0.
    fn cls(&mut self) {
        self.display.clear();
    }

    // Set pc to nnn, causing the program to jump to that memory location.
    fn jump(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // Set pc to last address popped from the stack.
    fn ret(&mut self) {
        self.pc = self.stack.pop();
    }

    // Push current pc to stack, and jump to nnn.
    fn call(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    // Set the register vx to nn.
    fn set_v(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
    }

    // Add the value nn to vx.
    //
    // In other instructions this would set the carry flag (vf) if the result
    // overflowed 8 bits. For this instruction, this is not the case.
    fn add(&mut self, x: usize, nn: u8) {
        self.v[x] = self.v[x].wrapping_add(nn)
    }

    // Set the register i to nnn.
    fn set_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    // Draw an n pixels tall and 8 pixels wide sprite at (vx, vy).
    fn draw(&mut self, x: usize, y: usize, n: u8) {
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
}
