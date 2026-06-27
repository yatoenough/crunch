pub mod gain;
pub mod modulation;

pub trait Effect: EffectClone + Send + Sync {
    fn process(&mut self, input_sample: f32) -> f32;
}

#[derive(Clone)]
pub struct EffectChain {
    effects: Vec<Box<dyn Effect>>,
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
