extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const FPS_RATE : u64 = 16;

const W_CHIP8 : usize = 64;
const H_CHIP8 : usize= 32;
const PIXEL_SIZE : usize = 10;

const WIDTH : usize = W_CHIP8 * PIXEL_SIZE;
const HEIGHT : usize = H_CHIP8 * PIXEL_SIZE;

pub const BLACK : u32 = 0x00_00_00u32;
pub const WHITE : u32 = 0xff_ff_ffu32;

pub struct GPU {
    buffer : Vec<u32>,
    pub window : Window
}

impl GPU {
    pub fn new() -> GPU {
        let mut win = Window::new(
            "CHIP8 EMULATOR", 
            WIDTH,
            HEIGHT,
            WindowOptions::default()
        ).expect("Impossible to initialize the window graphics");

        win.limit_update_rate(Some(std::time::Duration::from_millis(FPS_RATE)));

        GPU {
            buffer : vec![BLACK ; WIDTH * HEIGHT],
            window : win
        }
    }

    pub fn draw_pixel(&mut self, pos : (usize,usize) , color : u32) {
        for i in pos.0 * PIXEL_SIZE .. pos.0 * PIXEL_SIZE + PIXEL_SIZE {
            for j in pos.1 * PIXEL_SIZE .. pos.1 * PIXEL_SIZE + PIXEL_SIZE {
                self.buffer[ i + j * WIDTH] = color;
            }
        }
    } 

    pub fn clear_screen(&mut self) {
        self.buffer = vec![ BLACK ; WIDTH * HEIGHT ];
    }

    pub fn update_screen(&mut self) {
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT)
        .expect("Impossible to draw graphics");
    }

    pub fn must_continue(&self) -> bool {
        self.window.is_open() 
    }
} 
