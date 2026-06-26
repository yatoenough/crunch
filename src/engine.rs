use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::audio::pipeline::EffectChain;

pub struct AudioEngine {
    pub buffer: Arc<Mutex<VecDeque<f32>>>,
    pub effects: Arc<Mutex<EffectChain>>,
    pub input_channels: usize,
    pub output_channels: usize,
    pub sample_rate: f32,
}

impl AudioEngine {
    pub fn input_callback(&self) -> impl FnMut(&[f32], &cpal::InputCallbackInfo) + Send + 'static {
        let buffer = Arc::clone(&self.buffer);
        let effects = Arc::clone(&self.effects);
        let input_channels = self.input_channels;

        move |data, _| {
            let mut buf = buffer.lock().unwrap();

            for frame in data.chunks(input_channels) {
                let mut sample = frame.iter().sum::<f32>() / frame.len() as f32;

                if let Ok(mut fx) = effects.lock() {
                    sample = fx.process(sample);
                }

                buf.push_back(sample);
            }
        }
    }

    pub fn output_callback(
        &self,
    ) -> impl FnMut(&mut [f32], &cpal::OutputCallbackInfo) + Send + 'static {
        let buffer = Arc::clone(&self.buffer);
        let output_channels = self.output_channels;

        move |data, _| {
            let mut buf = buffer.lock().unwrap();

            for frame in data.chunks_mut(output_channels) {
                let sample = buf.pop_front().unwrap_or(0.0);
                for ch in frame {
                    *ch = sample;
                }
            }
        }
    }
}
