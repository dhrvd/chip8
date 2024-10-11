use emulator::Emulator;
use macroquad::prelude::*;

mod display;
mod emulator;
mod instructions;
mod memory;
mod stack;

use crate::display::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn window_conf() -> Conf {
    Conf {
        window_title: "CHIP8".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut emulator = Emulator::new(64);
    emulator.memory.load(include_bytes!("../IBM logo.ch8"));

    loop {
        emulator.cycle();
        emulator.display.draw();

        next_frame().await
    }
}
