pub struct DelayTimer {
    value: u8,
}

pub struct SoundTimer {
    value: u8,
    on_expired: dyn Fn() -> (),
}

impl Timer for SoundTimer {
    fn tick(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn on_timer_expired(&mut self, function: &dyn Fn() -> ()) {
        self.on_expired = *function.clone();
    }
}

trait Timer<T: FnMut()> {
    fn tick(&mut self);
    fn get_value(&self) -> u8;
    fn set_value(&mut self, value: u8);
    fn on_timer_expired(&mut self, function: &dyn Fn() -> ());
}

impl<T> Timer<T: FnMut()> for DelayTimer {
    fn tick(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn on_timer_expired(&mut self, _: &dyn Fn() -> ()) {}
}
