use crate::{Effect, EffectParam};

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
    fn name(&self) -> &'static str {
        "Chorus"
    }

    fn params(&self) -> Vec<EffectParam> {
        vec![
            EffectParam {
                name: "Rate",
                value: self.lfo_rate,
                min: 0.1,
                max: 8.0,
                step: 0.1,
            },
            EffectParam {
                name: "Depth",
                value: self.lfo_depth / self.sample_rate * 1000.0,
                min: 0.1,
                max: 10.0,
                step: 0.1,
            },
            EffectParam {
                name: "Delay",
                value: self.delay_center / self.sample_rate * 1000.0,
                min: 1.0,
                max: 40.0,
                step: 0.5,
            },
            EffectParam {
                name: "Mix",
                value: self.mix,
                min: 0.0,
                max: 1.0,
                step: 0.05,
            },
        ]
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "Rate" => self.lfo_rate = value.clamp(0.1, 8.0),
            "Depth" => self.lfo_depth = value.clamp(0.1, 10.0) / 1000.0 * self.sample_rate,
            "Delay" => self.delay_center = value.clamp(1.0, 40.0) / 1000.0 * self.sample_rate,
            "Mix" => self.mix = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

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
