#[derive(Default, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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

    pub fn set_pixel(&mut self, x: usize, y: usize, filled: bool) {
        let mut index: usize = Screen::WIDTH;
        index *= y;
        index += x;

        let pixel = &mut self.buffer[index];

        pixel.r = if filled { 255 } else { 0 };
        pixel.g = if filled { 255 } else { 0 };
        pixel.b = if filled { 255 } else { 0 };
        pixel.a = 255;
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
