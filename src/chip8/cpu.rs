const SIZE : usize = 4096;
const BEGIN_ADDR : u16 = 512;

pub struct CPU {
    memory : [ u8 ; SIZE ],
    v : [ u8 ; 16 ],
    i : u16, 
    jump : [ u16 ; 16 ],
    nb_jump : u8,
    count_game : u8,
    count_sound : u8,
    pc : u16
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
} 
