use pedalboard::Effect;

pub struct EffectChain {
    effects: Vec<Box<dyn Effect + Send>>,
}

impl EffectChain {
    pub fn new(effects: Vec<Box<dyn Effect + Send>>) -> Self {
        Self { effects }
    }

    pub fn process(&mut self, mut sample: f32) -> f32 {
        for effect in &mut self.effects {
            sample = effect.process(sample);
        }
        sample
    }
}
