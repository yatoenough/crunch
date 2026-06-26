use crate::Effect;

pub struct Overdrive {
    drive: f32,
    volume: f32,
}

impl Overdrive {
    pub fn new(drive: f32, volume: f32) -> Self {
        Self { drive, volume }
    }
}

impl Effect for Overdrive {
    fn process(&mut self, input_sample: f32) -> f32 {
        let driven = input_sample * self.drive;

        driven.tanh() * self.volume
    }
}
