const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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

pub struct Emulator {
    memory: [u8; 4096],
    display: [bool; 64 * 32],
    pc: u16, // program counter
    i: u16,  // index register
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16], // variable registers
    keypad: [bool; 16],
}

impl Emulator {
    pub fn new() -> Self {
        let mut emulator = Self {
            memory: [0; 4096],
            display: [false; 64 * 32],
            pc: 0x200,
            i: 0,
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            keypad: [false; 16],
        };

        // Load font data into memory
        emulator.memory[..80].copy_from_slice(&FONT);

        emulator
    }

    pub fn reset(&mut self) {
        self.memory = [0; 4096];
        self.display = [false; 64 * 32];
        self.pc = 0x200;
        self.i = 0;
        self.stack = Stack::new();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 16];

        self.memory[..80].copy_from_slice(&FONT);
    }
}
