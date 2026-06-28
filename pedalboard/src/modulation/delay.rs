use crate::{Effect, EffectParam};

#[derive(Clone)]
pub struct Delay {
    sample_rate: f32,
    buffer: Vec<f32>,
    write_ptr: usize,
    delay_time_ms: f32,
    delay_samples: usize,
    feedback: f32,
    wet: f32,
    dry: f32,
}

impl Delay {
    pub fn new(sample_rate: f32, delay_time_ms: f32, feedback: f32, wet: f32, dry: f32) -> Self {
        let delay_samples = ((delay_time_ms / 1000.0) * sample_rate) as usize;

        let buffer_size = delay_samples + 1;

        Self {
            sample_rate,
            buffer: vec![0.0; buffer_size],
            write_ptr: 0,
            delay_time_ms,
            delay_samples,
            feedback: feedback.clamp(0.0, 0.95),
            wet,
            dry,
        }
    }

    fn set_delay_time(&mut self, delay_time_ms: f32) {
        self.delay_time_ms = delay_time_ms.clamp(1.0, 2000.0);
        self.delay_samples = ((self.delay_time_ms / 1000.0) * self.sample_rate) as usize;
        self.buffer.resize(self.delay_samples + 1, 0.0);
        self.write_ptr %= self.buffer.len();
    }
}

impl Effect for Delay {
    fn name(&self) -> &'static str {
        "Delay"
    }

    fn params(&self) -> Vec<EffectParam> {
        vec![
            EffectParam {
                name: "Time",
                value: self.delay_time_ms,
                min: 1.0,
                max: 2000.0,
                step: 10.0,
            },
            EffectParam {
                name: "Feedback",
                value: self.feedback,
                min: 0.0,
                max: 0.95,
                step: 0.05,
            },
            EffectParam {
                name: "Wet",
                value: self.wet,
                min: 0.0,
                max: 1.0,
                step: 0.05,
            },
            EffectParam {
                name: "Dry",
                value: self.dry,
                min: 0.0,
                max: 1.0,
                step: 0.05,
            },
        ]
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "Time" => self.set_delay_time(value),
            "Feedback" => self.feedback = value.clamp(0.0, 0.95),
            "Wet" => self.wet = value.clamp(0.0, 1.0),
            "Dry" => self.dry = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        if self.delay_samples == 0 {
            return sample;
        }

        let read_ptr =
            (self.write_ptr + self.buffer.len() - self.delay_samples) % self.buffer.len();
        let delayed_sample = self.buffer[read_ptr];

        self.buffer[self.write_ptr] = sample + (delayed_sample * self.feedback);
        self.write_ptr = (self.write_ptr + 1) % self.buffer.len();

        (sample * self.dry) + (delayed_sample * self.wet)
    }
}
