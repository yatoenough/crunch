use crate::{Effect, EffectParam};

#[derive(Clone)]
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
    fn name(&self) -> &'static str {
        "Overdrive"
    }

    fn params(&self) -> Vec<EffectParam> {
        vec![
            EffectParam {
                name: "Drive",
                value: self.drive,
                min: 0.0,
                max: 30.0,
                step: 0.5,
            },
            EffectParam {
                name: "Volume",
                value: self.volume,
                min: 0.0,
                max: 1.0,
                step: 0.05,
            },
        ]
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "Drive" => self.drive = value.clamp(0.0, 30.0),
            "Volume" => self.volume = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        let driven = sample * self.drive;

        driven.tanh() * self.volume
    }
}
