use macroquad::prelude::*;

mod emulator;

fn window_conf() -> Conf {
    Conf {
        window_title: "CHIP8".to_owned(),
        window_height: 320,
        window_width: 640,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
