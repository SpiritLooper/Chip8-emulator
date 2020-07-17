extern crate minifb;

use minifb::{Key , Window};

pub fn keys_pressed(w : &Window) -> [bool ; 16] {
    let mut res = [false ; 16];
    
    res[0x0] = w.is_key_down(Key::NumPad0);
    res[0x1] = w.is_key_down(Key::NumPad7);
    res[0x2] = w.is_key_down(Key::NumPad8);
    res[0x3] = w.is_key_down(Key::NumPad9);
    res[0x4] = w.is_key_down(Key::NumPad4);
    res[0x5] = w.is_key_down(Key::NumPad5);
    res[0x6] = w.is_key_down(Key::NumPad6);
    res[0x7] = w.is_key_down(Key::NumPad1);
    res[0x8] = w.is_key_down(Key::NumPad2);
    res[0x9] = w.is_key_down(Key::NumPad3);
    res[0xa] = w.is_key_down(Key::Left);
    res[0xb] = w.is_key_down(Key::NumPadDot);
    res[0xc] = w.is_key_down(Key::NumPadAsterisk);
    res[0xd] = w.is_key_down(Key::NumPadMinus);
    res[0xe] = w.is_key_down(Key::NumPadPlus);
    res[0xf] = w.is_key_down(Key::NumPadEnter);

    res

}  
