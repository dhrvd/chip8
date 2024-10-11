use macroquad::prelude::*;

pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

pub const SCALE: u16 = 10;

pub const WINDOW_WIDTH: i32 = WIDTH as i32 * SCALE as i32;
pub const WINDOW_HEIGHT: i32 = HEIGHT as i32 * SCALE as i32;

pub const SIZE: usize = WIDTH as usize * HEIGHT as usize;

pub struct Display {
    frame_buffer: [bool; SIZE],
}

impl Display {
    pub fn new() -> Self {
        Self {
            frame_buffer: [false; SIZE],
        }
    }

    pub fn clear(&mut self) {
        self.frame_buffer = [false; SIZE]
    }

    fn index(x: u8, y: u8) -> usize {
        (x as usize) + (WIDTH as usize) * (y as usize)
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        self.frame_buffer[Self::index(x, y)]
    }

    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        self.frame_buffer[Self::index(x, y)] = value
    }

    pub fn draw(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.get(x, y) {
                    draw_rectangle(
                        x as f32 * SCALE as f32,
                        y as f32 * SCALE as f32,
                        SCALE as f32,
                        SCALE as f32,
                        WHITE,
                    );
                }
            }
        }
    }
}
