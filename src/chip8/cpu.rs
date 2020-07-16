const SIZE : usize = 4096;
const BEGIN_ADDR : usize = 512;

pub struct CPU {
    memory : [ u8 ; SIZE ],
    v : [ u8 ; 16 ],
    i : u16, 
    jump : [ u16 ; 16 ],
    nb_jump : u8,
    count_game : u8,
    count_sound : u8,
    pc : usize
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory : [ 0u8 ; SIZE ],
            v : [ 0u8 ; 16 ],
            i : 0u16, 
            jump : [ 0u16 ; 16 ],
            nb_jump : 0,
            count_game : 0,
            count_sound : 0,
            pc : BEGIN_ADDR 
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

        let nnn = opcode & 0x0FFF; 
        let nn = opcode & 0x00FF; 
        let n = opcode & 0x000F; 
        let x = opcode & 0x0F00; 
        let y = opcode & 0x00F0;
        
        let pc_change = match mask {
            // 00E0
            (0x0, 0x0, 0xe, 0x0) => unimplemented!("00E0") ,
            // 00EE
            (0x0, 0x0, 0xe, 0xe) => unimplemented!("00EE") ,
            // 0NNN
            (0x0, _, _, _) => unimplemented!("RCA Program") ,
            // 1NNN
            (0x1, _, _, _) => unimplemented!("1NNN") ,
            // 2NNN
            (0x2, _, _, _) => unimplemented!("2NNN") ,
            // 3XNN
            (0x3, _, _, _) => unimplemented!("3XNN") ,
            // 4XNN
            (0x4, _, _, _) => unimplemented!("4XNN") ,
            // 5XY0
            (0x5, _, _, 0x0) => unimplemented!("5XY0") ,
            // 6XNN
            (0x6, _, _, _) => unimplemented!("6XNN") ,
            // 7XNN
            (0x7, _, _, _) => unimplemented!("7XNN") ,
            // 8XY0
            (0x8, _, _, 0x0) => unimplemented!("8XY0") ,
            // 8XY1
            (0x8, _, _, 0x1) => unimplemented!("8XY1") ,
            // 8XY2
            (0x8, _, _, 0x2) => unimplemented!("8XY2") ,
            // 8XY3
            (0x8, _, _, 0x3) => unimplemented!("8XY3") ,
            // 8XY4
            (0x8, _, _, 0x4) => unimplemented!("8XY4") ,
            // 8XY5
            (0x8, _, _, 0x5) => unimplemented!("8XY5") ,
            // 8XY6
            (0x8, _, _, 0x6) => unimplemented!("8XY6") ,
            // 8XY7
            (0x8, _, _, 0x7) => unimplemented!("8XY7") ,
            // 8XYE
            (0x8, _, _, 0xe) => unimplemented!("8XYE") ,
            // 9XY0
            (0x9, _, _, 0x0) => unimplemented!("9XY0") ,
            // ANNN
            (0xa, _, _, _) => unimplemented!("ANNN") ,
            // BNNN
            (0xb, _, _, _) => unimplemented!("BNNN") ,
            // CXNN
            (0xc, _, _, _) => unimplemented!("CXNN") ,
            // DXYN
            (0xd, _, _, _) => unimplemented!("DXYN") ,
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
            (0xf, _, 0x3, 0x3) => unimplemented!("FX33") ,
            // FX55
            (0xf, _, 0x5, 0x5) => unimplemented!("FX55") ,
            // FX65
            (0xf, _, 0x6, 0x5) => unimplemented!("FX65") ,
            _ => panic!()
        };
    }
} 
