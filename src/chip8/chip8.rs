use crate::chip8::{gpu::*, cpu::*};


pub struct Chip8 {
    cpu : CPU,
    gpu : GPU
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu : CPU::new(),
            gpu : GPU::new()
        }
    }

    pub fn init(&mut self) {
        self.gpu.clear_screen();
        for x in 0 .. 64 {
            for y in 0 .. 32 {
                let color = match x % (y+1) {
                    0 => BLACK,
                    _ => WHITE
                };
                
                self.gpu.draw_pixel((x,y), color);
            }
        }
    }

    pub fn run(&mut self) {
        while self.gpu.must_continue() {
            self.gpu.update_screen();
        }
    }
} 