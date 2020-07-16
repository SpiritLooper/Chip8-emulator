pub mod chip8;

use chip8::cpu::* ;
use chip8::rom::* ;

fn main() {
       let args : Vec<String> = std::env::args().collect();

       let rom = Rom::new(&args[1]);

       let mut cpu = CPU::new();
       cpu.load(&rom);
       cpu.run();
}
