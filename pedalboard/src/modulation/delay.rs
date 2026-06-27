use crate::Effect;

#[derive(Clone)]
pub struct Delay {
    buffer: Vec<f32>,
    write_ptr: usize,
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
            buffer: vec![0.0; buffer_size],
            write_ptr: 0,
            delay_samples,
            feedback: feedback.clamp(0.0, 0.95),
            wet,
            dry,
        }
    }
}

impl Effect for Delay {
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
