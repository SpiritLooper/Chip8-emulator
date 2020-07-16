use crate::chip8::gpu::*;
use rand::Rng;

const SIZE : usize = 4096;
const BEGIN_ADDR : usize = 512;

enum OpCodeNext {
    Next,
    Skip,
    Jump(usize)
}

impl OpCodeNext {
    fn skip_if(condition : bool) -> OpCodeNext {
        if condition {
            OpCodeNext::Skip
        } else {
            OpCodeNext::Next
        }
    }
}

pub struct CPU {
    memory : [ u8 ; SIZE ],
    v : [ u8 ; 16 ],
    i : usize, 
    stack : [ usize ; 16 ],
    sp : usize,
    pc : usize,
    count_game : u8,
    count_sound : u8,
    gpu : GPU
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory : [ 0u8 ; SIZE ],
            v : [ 0u8 ; 16 ],
            i : 0, 
            stack : [ 0 ; 16 ],
            sp : 0,
            count_game : 0,
            count_sound : 0,
            pc : BEGIN_ADDR,
            gpu : GPU::new()
        }
    }

    pub fn step_count(&mut self) {
        if self.count_game > 0 {
            self.count_game -= 1;
        }

        if self.count_sound > 0 {
            self.count_sound -= 1;
        }
    }

    pub fn run(&mut self) {
        while self.gpu.must_continue() {
            self.gpu.update_screen();
        }
    }

    pub fn init(&mut self) {
        self.gpu.clear_screen();
    }

    pub fn get_opcode(&self) -> u16 {
        (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    pub fn run_opcode(&mut self, opcode : u16) {
        let mask = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );

        let nnn = ( opcode & 0x0FFF ) as usize; 
        let nn = ( opcode & 0x00FF ) as usize;
        let n = ( opcode & 0x000F ) as usize; 
        let x = ( opcode & 0x0F00 ) as usize; 
        let y = ( opcode & 0x00F0 ) as usize;
        
        let next_opcode = match mask {
            // 00E0
            (0x0, 0x0, 0xe, 0x0) => { 
                self.gpu.clear_screen() ; OpCodeNext::Next    
            } ,
            // 00EE
            (0x0, 0x0, 0xe, 0xe) => {
                self.sp -= 1;
                OpCodeNext::Jump(self.stack[self.sp])
            } ,
            // 0NNN
            (0x0, _, _, _) => unimplemented!("RCA Program") ,
            // 1NNN
            (0x1, _, _, _) => OpCodeNext::Jump(nnn) ,
            // 2NNN
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                OpCodeNext::Jump(nnn)
            } ,
            // 3XNN
            (0x3, _, _, _) => OpCodeNext::skip_if( self.v[x] == nn as u8 ) ,
            // 4XNN
            (0x4, _, _, _) => unimplemented!("4XNN") ,
            // 5XY0
            (0x5, _, _, 0x0) => unimplemented!("5XY0") ,
            // 6XNN
            (0x6, _, _, _) => unimplemented!("6XNN") ,
            // 7XNN
            (0x7, _, _, _) => unimplemented!("7XNN") ,
            // 8XY0
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
                OpCodeNext::Next
            } ,
            // 8XY1
            (0x8, _, _, 0x1) => unimplemented!("8XY1") ,
            // 8XY2
            (0x8, _, _, 0x2) => unimplemented!("8XY2") ,
            // 8XY3
            (0x8, _, _, 0x3) => unimplemented!("8XY3") ,
            // 8XY4
            (0x8, _, _, 0x4) => {
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let result = vx + vy;
                self.v[x] = result as u8;
                self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
                OpCodeNext::Next
            } ,
            // 8XY5
            (0x8, _, _, 0x5) => unimplemented!("8XY5") ,
            // 8XY6
            (0x8, _, _, 0x6) => unimplemented!("8XY6") ,
            // 8XY7
            (0x8, _, _, 0x7) => {
                self.v[0xf] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x]   = self.v[y].wrapping_sub(self.v[x]);
                OpCodeNext::Next
            } ,
            // 8XYE
            (0x8, _, _, 0xe) => unimplemented!("8XYE") ,
            // 9XY0
            (0x9, _, _, 0x0) => unimplemented!("9XY0") ,
            // ANNN
            (0xa, _, _, _) => unimplemented!("ANNN") ,
            // BNNN
            (0xb, _, _, _) => unimplemented!("BNNN") ,
            // CXNN
            (0xc, _, _, _) => {
                let mut rng = rand::thread_rng();
                self.v[x] = rng.gen_range(0,nn) as u8;
                OpCodeNext::Next
            } ,
            // DXYN
            (0xd, _, _, _) => self.draw_sprite(x, y, n) ,
            // EX9E
            (0xe, _, 0x9, 0xe) => unimplemented!("EX9E") ,
            // EXA1
            (0xe, _, 0xa, 0x1) => unimplemented!("EXA1") ,
            // FX07
            (0xf, _, 0x0, 0x7) => unimplemented!("FX07") ,
            // FX0A
            (0xf, _, 0x0, 0xa) => unimplemented!("FX0A") ,
            // FX15
            (0xf, _, 0x1, 0x5) => unimplemented!("FX15") ,
            // FX18
            (0xf, _, 0x1, 0x8) => unimplemented!("FX18") ,
            // FX1E
            (0xf, _, 0x1, 0xe) => unimplemented!("FX1E") ,
            // FX29
            (0xf, _, 0x2, 0x9) => unimplemented!("FX29") ,
            // FX33
            (0xf, _, 0x3, 0x3) => {
                self.memory[self.i] = self.v[x] / 100;
                self.memory[self.i + 1] = (self.v[x] % 100) / 10;
                self.memory[self.i + 2] = self.v[x] % 10;
                OpCodeNext::Next
            },
            // FX55
            (0xf, _, 0x5, 0x5) => unimplemented!("FX55") ,
            // FX65
            (0xf, _, 0x6, 0x5) => unimplemented!("FX65") ,
            _ => panic!()
        };

        match next_opcode {
            OpCodeNext::Next => self.pc += 2,
            OpCodeNext::Skip => self.pc += 2 * 2,
            OpCodeNext::Jump(addr) => self.pc = addr
        }
    }

    fn draw_sprite(&mut self, x : usize, y : usize , n : usize  ) -> OpCodeNext {
        self.v[0xf] = 0;
        for byte in 0 .. n {
            let y = ( self.v[y] as usize + byte ) % H_CHIP8;
            for bit in 0 .. 8 {
                let x = (self.v[x] as usize + bit ) % W_CHIP8;
                let val_bit = self.memory[self.i + byte] >> (7 - bit) & 1;
                if val_bit != 0 {
                    let mut color = WHITE;
                    if self.gpu.get_color((x,y)) == WHITE {
                        color = BLACK;
                    }
                    self.gpu.draw_pixel( (x,y), color );
                }
            }
        }
        OpCodeNext::Next
    }
} 
