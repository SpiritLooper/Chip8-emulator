mod chip8;

use chip8::chip8::*;

fn main() {
       let mut chip8 = Chip8::new();
       chip8.init();
       chip8.run();
}
