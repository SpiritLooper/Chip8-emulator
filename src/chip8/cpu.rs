use crate::chip8::gpu::*;
use super::font::FONT_SET;
use super::rom::Rom ;
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
            let opcode = self.get_opcode();
            self.run_opcode(opcode);
            self.gpu.update_screen();
        }
    }

    pub fn load(&mut self, rom : &Rom) {
        // Load font 
        for i in 0 .. FONT_SET.len() {
            self.memory[i] = FONT_SET[i];
        }

        // Load ROM
        for ( i , &byte ) in rom.data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < SIZE {
                self.memory[addr] = byte;
            } else {
                break;
            }
        }

        //Clear screen
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
            (0x0, _, _, _) => panic!("RCA Program") ,
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
            (0x4, _, _, _) => OpCodeNext::skip_if( self.v[x] != nn as u8 ) ,
            // 5XY0
            (0x5, _, _, 0x0) => OpCodeNext::skip_if( self.v[x] == self.v[y] as u8 ) ,
            // 6XNN
            (0x6, _, _, _) => { self.v[x] = nn as u8 ; OpCodeNext::Next } ,
            // 7XNN
            (0x7, _, _, _) => { self.v[x] += nn as u8 ; OpCodeNext::Next } ,
            // 8XY0
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
                OpCodeNext::Next
            } ,
            // 8XY1
            (0x8, _, _, 0x1) => { self.v[x] |= self.v[y] ; OpCodeNext::Next } ,
            // 8XY2
            (0x8, _, _, 0x2) => { self.v[x] &= self.v[y] ; OpCodeNext::Next } ,
            // 8XY3
            (0x8, _, _, 0x3) => { self.v[x] ^= self.v[y] ; OpCodeNext::Next } ,
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
            (0x8, _, _, 0x5) => {  
                self.v[0xf] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                OpCodeNext::Next
            } ,
            // 8XY6
            (0x8, _, _, 0x6) => {
                self.v[0xf] = self.v[x] & 0x1;
                self.v[x] >>= 1;
                OpCodeNext::Next
            } ,
            // 8XY7
            (0x8, _, _, 0x7) => {
                self.v[0xf] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x]   = self.v[y].wrapping_sub(self.v[x]);
                OpCodeNext::Next
            } ,
            // 8XYE
            (0x8, _, _, 0xe) => { 
                self.v[0xf] = ( self.v[x] & 0b1000_0000 ) >> 7;
                self.v[x] <<= 1;
                OpCodeNext::Next
            } ,
            // 9XY0
            (0x9, _, _, 0x0) =>  OpCodeNext::skip_if(self.v[x] != self.v[y]),
            // ANNN
            (0xa, _, _, _) => { self.i = nnn ; OpCodeNext::Next },
            // BNNN
            (0xb, _, _, _) => OpCodeNext::Jump(nnn + self.v[0x0] as usize),
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
            (0xf, _, 0x0, 0x7) => { self.v[x] = self.count_game ; OpCodeNext::Next } ,
            // FX0A
            (0xf, _, 0x0, 0xa) => unimplemented!("FX0A") ,
            // FX15
            (0xf, _, 0x1, 0x5) => { self.count_game = self.v[x] ; OpCodeNext::Next } ,
            // FX18
            (0xf, _, 0x1, 0x8) => { self.count_sound = self.v[x] ; OpCodeNext::Next } ,
            // FX1E
            (0xf, _, 0x1, 0xe) => {
                self.i += self.v[x] as usize;
                self.v[0xf] = if self.i > 0xf00 { 1 } else { 0 };
                OpCodeNext::Next
            } ,
            // FX29
            (0xf, _, 0x2, 0x9) => {
                self.i = 5 * self.v[x] as usize;
                OpCodeNext::Next
            } ,
            // FX33
            (0xf, _, 0x3, 0x3) => {
                self.memory[self.i] = self.v[x] / 100;
                self.memory[self.i + 1] = (self.v[x] % 100) / 10;
                self.memory[self.i + 2] = self.v[x] % 10;
                OpCodeNext::Next
            },
            // FX55
            (0xf, _, 0x5, 0x5) => {
                for i in 0 .. x + 1 {
                    self.memory[self.i + i] = self.v[i];
                }
                OpCodeNext::Next
            } ,
            // FX65
            (0xf, _, 0x6, 0x5) => {
                for i in 0 .. x + 1 {
                    self.v[i] = self.memory[self.i + i];
                }
                OpCodeNext::Next
            } ,
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
