pub mod chip8;

use chip8::gpu;

fn main() {
    let mut gpu = gpu::GPU::new();
    gpu.clear_screen();
    for x in 0 .. 64 {
        for y in 0 .. 32 {
            let color = match x % (y+1) {
                0 => gpu::BLACK,
                _ => gpu::WHITE
            };
            
            gpu.draw_pixel((x,y), color);
        }
    }

    while gpu.window.is_open() {
        gpu.update_screen();
    }
}
