use std::time::Duration;

use display::{HEIGHT, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use emulator::Emulator;

use sdl2::{event::Event, keyboard::Keycode};

mod display;
mod emulator;
mod instructions;
mod keypad;
mod memory;
mod stack;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();

    canvas
        .set_logical_size(WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = Emulator::new(8);
    emulator.memory.load(include_bytes!("../roms/BRIX"));

    'running: loop {
        match event_pump.poll_event() {
            Some(Event::Quit { .. })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            }) => break 'running,
            Some(Event::KeyDown {
                scancode: Some(scancode),
                ..
            }) => {
                emulator.keypad.update(scancode, true);
            }
            Some(Event::KeyUp {
                scancode: Some(scancode),
                ..
            }) => {
                emulator.keypad.update(scancode, false);
            }
            _ => {}
        }

        emulator.update();
        emulator.display.draw(&mut canvas);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }
}
