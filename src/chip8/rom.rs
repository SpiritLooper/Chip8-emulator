use std::fs::File;
use std::io::prelude::*;

const MAX_SIZE : usize =  4096 - 512 ;

pub struct Rom {
    pub data : [ u8 ; MAX_SIZE ],
    pub size : usize
}

impl Rom {
    pub fn new(filename : &str) -> Self {
        
        let mut file = File::open(filename).expect("File not found");

        let mut buffer = [ 0u8 ; MAX_SIZE ];

        let bytes_read = match file.read(&mut buffer) {
            Ok(len) => len, 
            _ => panic!("Error when loading file")
        };

        Rom {
            data : buffer,
            size : bytes_read
        }
    }
} 