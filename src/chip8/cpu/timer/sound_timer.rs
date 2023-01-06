use super::timer::Timer;

pub struct SoundTimer {
    value: u8,
    active: bool,
}

impl SoundTimer {
    pub fn new() -> Self {
        Self {
            value: 0,
            active: false,
        }
    }
}

impl Timer for SoundTimer {
    fn tick(&mut self) {
        if !self.active {
            return;
        }
        self.value = self.value.saturating_sub(1);
        if self.value == 0 {
            self.active = false;
        }
    }

    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.active = true;
        self.value = value;
    }
}
