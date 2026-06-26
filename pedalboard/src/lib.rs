pub mod gain;
pub mod modulation;

pub trait Effect: Send + Sync {
    fn process(&mut self, input_sample: f32) -> f32;
}

pub struct EffectChain {
    effects: Vec<Box<dyn Effect>>,
}

impl EffectChain {
    pub fn new(effects: Vec<Box<dyn Effect>>) -> Self {
        Self { effects }
    }
}

impl Effect for EffectChain {
    fn process(&mut self, mut sample: f32) -> f32 {
        for effect in &mut self.effects {
            sample = effect.process(sample);
        }
        sample
    }
}
