use crate::chip8::cpu::*;

pub struct Chip8 {
    cpu : CPU
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu : CPU::new()
        }
    }

    pub fn init(&mut self) {
        self.cpu.init();
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
} 