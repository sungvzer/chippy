#[derive(Default, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn filled(&self) -> bool {
        self.r == 255 && self.g == 255 && self.b == 255
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
        let pixel = Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
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
        self.fill({
            Pixel {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        })
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

        existing_pixel.r = if fill_pixel { 255 } else { 0 };
        existing_pixel.g = if fill_pixel { 255 } else { 0 };
        existing_pixel.b = if fill_pixel { 255 } else { 0 };
        existing_pixel.a = 255;
        did_erase_pixel
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let px = self.buffer[i];

            pixel.copy_from_slice(&[px.r, px.g, px.b, px.a]);
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
