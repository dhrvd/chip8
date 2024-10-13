use crate::{display::Display, memory::Memory, stack::Stack};

pub const START_ADDR: u16 = 0x200;

pub struct Emulator {
    frequency: f32,
    accumulator: f32,

    pub memory: Memory,
    pub display: Display,
    pub pc: u16, // program counter
    pub i: u16,  // index register
    pub stack: Stack,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v: [u8; 16], // variable registers

    pub keypad: [bool; 16],
}

impl Emulator {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            accumulator: 0.0,
            memory: Memory::new(),
            display: Display::new(),
            pc: START_ADDR,
            i: 0,
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            keypad: [false; 16],
        }
    }

    fn reset(&mut self) {
        self.memory = Memory::new();
        self.display = Display::new();
        self.pc = START_ADDR;
        self.i = 0;
        self.stack = Stack::new();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 16];
    }

    // fetch the instruction from memory at current program counter
    fn fetch(&mut self) -> u16 {
        let instruction = ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16;
        self.pc += 2;

        instruction
    }

    fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: beep
            }

            self.sound_timer -= 1;
        }
    }

    pub fn cycle(&mut self) {
        self.tick_timers();

        let instruction = self.fetch();
        // println!("0x{:04X}", instruction);
        self.execute(instruction);
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;

        while self.accumulator >= 1.0 / self.frequency {
            self.cycle();
            self.accumulator -= 1.0 / self.frequency;
        }
    }

    fn keypress(&mut self, index: usize, pressed: bool) {
        self.keypad[index] = pressed;
    }
}
