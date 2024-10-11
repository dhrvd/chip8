pub struct Stack {
    stack: [u16; 16],
    pointer: u16,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; 16],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.pointer as usize] = value;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        self.stack[self.pointer as usize]
    }
}
