use std::sync::mpsc::Receiver;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Host, SupportedStreamConfig,
};
use log::{debug, error};

use super::message::SoundMessage;

pub struct Sound {
    _host: Host,
    config: SupportedStreamConfig,
    message_rx: Receiver<SoundMessage>,
    playing: bool,
}

impl Sound {
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn new(message_rx: Receiver<SoundMessage>) -> Self {
        let host = cpal::default_host();

        let device = host.default_output_device().unwrap();

        let config = device.default_output_config().unwrap();
        debug!("Default output config: {:?}", config);
        let mut sound = Sound {
            _host: host,
            config,
            message_rx,
            playing: false,
        };

        sound.exec();
        sound
    }

    pub fn exec(&mut self) {
        let config_copy = self.config.clone();

        let device = self._host.default_output_device().unwrap();
        debug!("Output device: {}", device.name().unwrap());

        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run::<f32>(&device, &config_copy.into()),
            cpal::SampleFormat::I16 => self.run::<i16>(&device, &config_copy.into()),
            cpal::SampleFormat::U16 => self.run::<u16>(&device, &config_copy.into()),
        };
    }

    pub fn run<T>(&mut self, device: &cpal::Device, config: &cpal::StreamConfig) -> ()
    where
        T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 220.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| error!("an error occurred on stream: {}", err);

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    Self::write_data(data, channels, &mut next_value)
                },
                err_fn,
            )
            .unwrap();
        stream.pause().unwrap();

        loop {
            if let Ok(message) = self.message_rx.try_recv() {
                match message {
                    SoundMessage::Play if self.playing == false => {
                        debug!("Received play message");
                        stream.play().unwrap_or_else(|err| {
                            error!("Error with PlayStream: {:?}", err);
                        });
                        self.playing = true;
                    }
                    SoundMessage::Pause if self.playing == true => {
                        debug!("Received pause message");

                        stream.pause().unwrap_or_else(|err| {
                            error!("Error with PauseStream: {:?}", err);
                        });
                        self.playing = false;
                    }
                    SoundMessage::Stop => {
                        return;
                    }
                    _ => {
                        debug!("Ignoring no-ops");
                    }
                }
            }
        }
    }

    pub fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}
