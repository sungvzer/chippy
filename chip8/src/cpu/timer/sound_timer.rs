use std::sync::mpsc::Sender;

use log::error;

use crate::sound::message::SoundMessage;

use super::timer::Timer;

pub struct SoundTimer {
    value: u8,
    active: bool,
    sound_tx: Sender<SoundMessage>,
}

impl SoundTimer {
    pub fn new(sound_tx: Sender<SoundMessage>) -> Self {
        Self {
            value: 0,
            active: false,
            sound_tx,
        }
    }

    pub fn force_audio_stop(&self) {
        self.sound_tx
            .send(SoundMessage::Stop)
            .unwrap_or_else(|err| {
                error!("Error while stopping sound! {:?}", err.to_string());
            });
    }
}

impl Timer for SoundTimer {
    fn tick(&mut self) {
        if !self.active {
            return;
        }
        self.value = self.value.saturating_sub(1);
        if self.value == 0 {
            self.sound_tx
                .send(SoundMessage::Pause)
                .unwrap_or_else(|err| {
                    error!("Error pausing sound: {:?}", err);
                });
            self.active = false;
        }
    }

    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.active = true;
        self.value = value;
        self.sound_tx
            .send(SoundMessage::Play)
            .unwrap_or_else(|err| {
                error!("Error playing sound: {:?}", err);
            });
    }
}
