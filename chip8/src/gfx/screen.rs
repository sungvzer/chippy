pub struct Screen {
    buffer: [u8; (Screen::WIDTH / 8) * (Screen::HEIGHT / 8)],
    changed: bool,
}

impl Screen {
    pub const HEIGHT: usize = 32;
    pub const WIDTH: usize = 64;
}

impl Screen {
    pub fn new() -> Self {
        const HEIGHT_BYTES: usize = Screen::HEIGHT / 8;
        const WIDTH_BYTES: usize = Screen::WIDTH / 8;
        Screen {
            buffer: [0x00; WIDTH_BYTES * HEIGHT_BYTES],
            changed: false,
        }
    }
    /// Returns `true` if a filled pixel has been erased
    pub fn draw_sprite(&mut self, mut x: usize, mut y: usize, sprite: &Vec<u8>) -> bool {
        let mut did_erase_pixel = false;
        x = x % Screen::WIDTH;
        y = y % Screen::HEIGHT;

        // For every byte (0b01010101)
        for sprite_byte in sprite {
            if *sprite_byte != 0 {
                println!("Drawing 0b{:b}", *sprite_byte);
            }
            let index = (y * Screen::WIDTH) + x;
            println!("Converting {x},{y} to {index}");
            let current_byte = self.buffer.get_mut(index).unwrap();
            did_erase_pixel = sprite_byte & *current_byte != 0;

            *current_byte ^= sprite_byte;
            y += 1;
        }
        self.changed = true;
        did_erase_pixel
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0x00);
        self.changed = true;
    }

    pub fn fill(&mut self, fill: bool) {
        self.buffer.fill(if fill { 0xFF } else { 0x00 });
    }

    pub fn draw(&mut self, frame: &mut [u8]) -> bool {
        if !self.changed {
            return false;
        }

        for (index, px) in self.buffer.iter().enumerate() {
            for j in 0..8 {
                let slice: [u8; 4] = if px & (1 << j) != 0 {
                    [0xff, 0xff, 0xff, 0xff]
                } else {
                    [0, 0, 0, 0]
                };
                frame.chunks_exact_mut(4).collect::<Vec<&mut u8>>();
            }
        }

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let buffer_index = i / 8;
            let px = self.buffer[buffer_index];
            for j in 0..8 {
                let slice: [u8; 4] = if px & (1 << j) != 0 {
                    [0xff, 0xff, 0xff, 0xff]
                } else {
                    [0, 0, 0, 0]
                };
                pixel.copy_from_slice(&slice);
            }
        }
        self.changed = false;
        true
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
