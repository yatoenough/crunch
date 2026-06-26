use crate::effects::Effect;

pub struct Overdrive {
    gain: f32,
    volume: f32,
}

impl Overdrive {
    pub fn new(gain: f32, volume: f32) -> Self {
        Self { gain, volume }
    }
}

impl Effect for Overdrive {
    fn process(&mut self, input_sample: f32) -> f32 {
        let driven = input_sample * self.gain;
        let distorted = driven.tanh();

        distorted * self.volume
    }
}
