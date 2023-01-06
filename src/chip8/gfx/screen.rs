use log::debug;

#[derive(Default, Clone, Copy)]
pub struct Pixel {
    filled: bool,
}

impl Pixel {
    pub fn filled(&self) -> bool {
        self.filled
    }

    pub fn set_filled(&mut self, filled: bool) {
        self.filled = filled;
    }

    pub fn new() -> Self {
        Pixel { filled: false }
    }
}
pub struct Screen {
    buffer: [Pixel; Screen::WIDTH * Screen::HEIGHT],
}

impl Screen {
    pub const HEIGHT: usize = 32;
    pub const WIDTH: usize = 64;
}

impl Screen {
    pub fn new() -> Self {
        let pixel = Pixel { filled: false };
        Screen {
            buffer: [pixel; Screen::WIDTH * Screen::HEIGHT],
        }
    }
    /// Returns `true` if a filled pixel has been erased
    pub fn draw_sprite(&mut self, x: usize, mut y: usize, sprite: &Vec<u8>) -> bool {
        let mut did_erase_pixel = false;

        for byte in sprite {
            for j in 0..8 {
                let filled = (byte & (1 << j)) != 0;
                did_erase_pixel = did_erase_pixel || self.set_pixel(x + (7 - j), y, filled);
            }

            y += 1;
        }
        false
    }

    pub fn clear(&mut self) {
        self.fill(Pixel::new())
    }

    pub fn fill(&mut self, pixel: Pixel) {
        self.buffer.fill(pixel);
    }

    /// Returns `true` if a filled pixel has been erased
    pub fn set_pixel(&mut self, x: usize, y: usize, mut fill_pixel: bool) -> bool {
        let mut did_erase_pixel = false;
        let mut index: usize = Screen::WIDTH;
        index *= y;
        index += x;

        // 0 XOR 0 is not of interest as it does not cause any
        let existing_pixel = &mut self.buffer[index];
        if existing_pixel.filled() && fill_pixel {
            fill_pixel = false;
            did_erase_pixel = true;
        }

        existing_pixel.set_filled(fill_pixel);
        did_erase_pixel
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let px = self.buffer[i];

            let slice: [u8; 4] = if px.filled {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0, 0, 0, 0]
            };
            pixel.copy_from_slice(&slice);
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
