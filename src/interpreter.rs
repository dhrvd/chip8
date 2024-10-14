use crate::{display::Display, keypad::Keypad, memory::Memory, stack::Stack};
use rand::rngs::ThreadRng;

pub const START_ADDR: u16 = 0x200;

pub struct Interpreter {
    cycles_per_frame: u16,
    pub thread_range: ThreadRng,

    pub memory: Memory,
    pub display: Display,
    pub pc: u16, // program counter
    pub i: u16,  // index register
    pub stack: Stack,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v: [u8; 16], // variable registers

    pub keypad: Keypad,
}

impl Interpreter {
    pub fn new(cycles_per_frame: u16) -> Self {
        Self {
            cycles_per_frame,
            thread_range: rand::thread_rng(),
            memory: Memory::new(),
            display: Display::new(),
            pc: START_ADDR,
            i: 0,
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            keypad: Keypad::new(),
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
        self.keypad = Keypad::new();
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
        if self.keypad.waiting {
            if !self.keypad[self.keypad.down] {
                self.keypad.waiting = false;
                self.v[self.keypad.register] = self.keypad.down;
            }
        } else {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }

    pub fn update(&mut self) {
        for _ in 0..self.cycles_per_frame {
            self.cycle();
        }

        self.tick_timers();
    }
}
