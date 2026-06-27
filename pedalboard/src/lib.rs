pub mod gain;
pub mod modulation;

pub trait Effect: EffectClone + Send + Sync {
    fn process(&mut self, input_sample: f32) -> f32;
}

pub trait EffectClone {
    fn clone_box(&self) -> Box<dyn Effect>;
}

impl<T> EffectClone for T
where
    T: 'static + Effect + Clone,
{
    fn clone_box(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Default)]
pub struct EffectChain {
    effects: Vec<Box<dyn Effect>>,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn apply<T: Effect + 'static>(mut self, effect: T) -> Self {
        self.effects.push(Box::new(effect));
        self
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
