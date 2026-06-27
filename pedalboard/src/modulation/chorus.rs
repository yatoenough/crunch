use crate::Effect;

#[derive(Clone)]
pub struct Chorus {
    sample_rate: f32,
    buffer: Vec<f32>,
    write_pos: usize,

    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    delay_center: f32,

    mix: f32,
}

impl Chorus {
    pub fn new(sample_rate: f32) -> Self {
        let max_delay_samples = (sample_rate * 0.05) as usize;
        Self {
            sample_rate,
            buffer: vec![0.0; max_delay_samples],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_rate: 0.5,
            lfo_depth: sample_rate * 0.002,
            delay_center: sample_rate * 0.015,
            mix: 0.5,
        }
    }
}

impl Effect for Chorus {
    fn process(&mut self, sample: f32) -> f32 {
        self.buffer[self.write_pos] = sample;

        self.lfo_phase += self.lfo_rate / self.sample_rate;

        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let lfo = (self.lfo_phase * std::f32::consts::TAU).sin();

        let delay_samples = self.delay_center + lfo * self.lfo_depth;
        let read_offset = delay_samples.max(1.0);

        let n = self.buffer.len() as f32;
        let read_pos = (self.write_pos as f32 - read_offset).rem_euclid(n);
        let i0 = (read_pos as usize) % self.buffer.len();
        let i1 = (i0 + 1) % self.buffer.len();
        let frac = read_pos.fract();

        let wet = self.buffer[i0] + frac * (self.buffer[i1] - self.buffer[i0]);

        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        sample * (1.0 - self.mix) + wet * self.mix
    }
}
