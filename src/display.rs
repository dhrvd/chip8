use sdl2::{pixels::Color, rect::Point, render::WindowCanvas};

pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

const SCALE: u16 = 10;

pub const WINDOW_WIDTH: u32 = WIDTH as u32 * SCALE as u32;
pub const WINDOW_HEIGHT: u32 = HEIGHT as u32 * SCALE as u32;

const SIZE: usize = WIDTH as usize * HEIGHT as usize;

const BACKGROUND: Color = Color::RGB(30, 32, 30);
const FOREGROUND: Color = Color::RGB(236, 223, 204);

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

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(BACKGROUND);
        canvas.clear();
        canvas.set_draw_color(FOREGROUND);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.get(x, y) {
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                }
            }
        }

        canvas.present();
    }
}
