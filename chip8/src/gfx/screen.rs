pub struct Screen {
    buffer: [u8; Screen::WIDTH * Screen::HEIGHT],
    changed: bool,
}

impl Screen {
    pub const HEIGHT: usize = 32;
    pub const WIDTH: usize = 64;
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            buffer: [0x00; Self::WIDTH * Self::HEIGHT],
            changed: false,
        }
    }
    /// Returns `true` if a filled pixel has been erased
    pub fn draw_sprite(&mut self, mut x: usize, mut y: usize, sprite: &Vec<u8>) -> bool {
        let mut did_erase_pixel = false;
        log::debug!("Sprite: {:0x?}", sprite);

        x %= Screen::WIDTH;
        y %= Screen::HEIGHT;

        // For every byte (0b01010101) in the sprite, we need to write 8 bytes into our memory
        for sprite_byte in sprite {
            // Convert the (x,y) into a 1D index
            let index = (y * Screen::WIDTH) + x;

            // For every bit
            for bit in 0..8 {
                // Find out whether it's a 1 or a 0 (white or black pixel)
                let mask = 1 << bit;
                let masked_sprite_byte = sprite_byte & mask;
                let draw_byte: u8 = if masked_sprite_byte != 0 { 0xff } else { 0x00 };

                // "Mirror effect", draw from index + 7 to index
                let index = index + (7 - bit);

                // VF calculation and XOR onto screen
                did_erase_pixel |= (self.buffer[index] == 0xff) && draw_byte == 0xff;
                self.buffer[index] ^= draw_byte;
            }
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

        let mut pixel_vector: Vec<&mut [u8]> = frame.chunks_exact_mut(4).collect();

        // For every pixel (byte)
        for (index, px) in self.buffer.iter().enumerate() {
            // Pick the individual bit
            let slice: [u8; 4] = if *px == 0xff {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0, 0, 0, 0]
            };
            pixel_vector[index].copy_from_slice(&slice);
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
