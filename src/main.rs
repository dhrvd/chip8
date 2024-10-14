use display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use emulator::Emulator;
use macroquad::prelude::*;

mod display;
mod emulator;
mod instructions;
mod keypad;
mod memory;
mod stack;

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
    let mut emulator = Emulator::new(500);
    emulator.memory.load(include_bytes!("../roms/6-keypad.ch8"));

    loop {
        emulator.update();
        emulator.display.draw();

        next_frame().await
    }
}
