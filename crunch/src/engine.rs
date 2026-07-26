use std::sync::{Arc, Mutex};
use std::time::Duration;

use cpal::{Device, Stream, StreamConfig, traits::DeviceTrait, traits::StreamTrait};
use pedalboard::{Effect, EffectChain};

use crate::audio::buffer::AudioBuffer;

const BUFFER_CAPACITY: usize = 8192;

pub struct AudioEngine {
    pub buffer: Arc<Mutex<AudioBuffer>>,
    pub effects: Arc<Mutex<EffectChain>>,
    pub input_channels: usize,
    pub output_channels: usize,
    pub sample_rate: f32,
}

impl AudioEngine {
    pub fn new(
        input_config: &StreamConfig,
        output_config: &StreamConfig,
        effects: Arc<Mutex<EffectChain>>,
    ) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(AudioBuffer::new(BUFFER_CAPACITY))),
            effects,
            input_channels: input_config.channels as usize,
            output_channels: output_config.channels as usize,
            sample_rate: output_config.sample_rate as f32,
        }
    }

    pub fn input_callback(&self) -> impl FnMut(&[f32], &cpal::InputCallbackInfo) + Send + 'static {
        let buffer = Arc::clone(&self.buffer);
        let effects = Arc::clone(&self.effects);
        let input_channels = self.input_channels;

        move |data, _| {
            let mut buf = match buffer.try_lock() {
                Ok(b) => b,
                Err(_) => return,
            };

            for frame in data.chunks(input_channels) {
                let mut sample = frame.iter().sum::<f32>() / frame.len() as f32;

                if let Ok(mut fx) = effects.try_lock() {
                    sample = fx.process(sample);
                }

                buf.push(sample);
            }
        }
    }

    pub fn output_callback(
        &self,
    ) -> impl FnMut(&mut [f32], &cpal::OutputCallbackInfo) + Send + 'static {
        let buffer = Arc::clone(&self.buffer);
        let output_channels = self.output_channels;

        move |data, _| {
            let mut buf = match buffer.try_lock() {
                Ok(b) => b,
                Err(_) => {
                    for ch in data.iter_mut() {
                        *ch = 0.0;
                    }
                    return;
                }
            };

            for frame in data.chunks_mut(output_channels) {
                let sample = buf.pop();
                for ch in frame {
                    *ch = sample;
                }
            }
        }
    }
}

pub struct AudioStreams {
    buffer: Arc<Mutex<AudioBuffer>>,
    input_stream: Stream,
    output_stream: Stream,
    target_fill: usize,
}

impl AudioStreams {
    pub fn new(
        input_device: &Device,
        output_device: &Device,
        input_config: StreamConfig,
        output_config: StreamConfig,
        effects: Arc<Mutex<EffectChain>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = AudioEngine::new(&input_config, &output_config, effects);
        let buffer = Arc::clone(&engine.buffer);
        let target_fill = (output_config.sample_rate as f32 * 0.030) as usize;

        let input_stream = input_device.build_input_stream(
            input_config,
            engine.input_callback(),
            |e| eprintln!("{e}"),
            None,
        )?;

        let output_stream = output_device.build_output_stream(
            output_config,
            engine.output_callback(),
            |e| eprintln!("{e}"),
            None,
        )?;

        Ok(Self {
            buffer,
            input_stream,
            output_stream,
            target_fill,
        })
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.input_stream.play()?;

        let poll_interval = Duration::from_millis(2);
        let mut waited = 0u32;
        let max_wait = 100;
        while waited < max_wait {
            let fill = self.buffer.lock().unwrap().len();
            if fill >= self.target_fill {
                break;
            }
            std::thread::sleep(poll_interval);
            waited += 1;
        }

        self.output_stream.play()?;
        Ok(())
    }

    pub fn pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.output_stream.pause()?;
        self.input_stream.pause()?;
        Ok(())
    }
}
