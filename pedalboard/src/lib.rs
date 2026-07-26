pub mod gain;
pub mod modulation;

#[derive(Clone)]
pub struct EffectParam {
    pub name: &'static str,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

pub trait Effect: EffectClone + Send + Sync {
    fn name(&self) -> &'static str;
    fn params(&self) -> Vec<EffectParam>;
    fn set_param(&mut self, name: &str, value: f32);
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

    pub fn effects(&self) -> &[Box<dyn Effect>] {
        &self.effects
    }

    pub fn effects_mut(&mut self) -> &mut [Box<dyn Effect>] {
        &mut self.effects
    }

    pub fn push_effect(&mut self, effect: Box<dyn Effect>) {
        self.effects.push(effect);
    }

    pub fn remove_effect(&mut self, index: usize) {
        if index < self.effects.len() {
            self.effects.remove(index);
        }
    }
}

impl Effect for EffectChain {
    fn name(&self) -> &'static str {
        "Effect Chain"
    }

    fn params(&self) -> Vec<EffectParam> {
        Vec::new()
    }

    fn set_param(&mut self, _name: &str, _value: f32) {}

    fn process(&mut self, mut sample: f32) -> f32 {
        for effect in &mut self.effects {
            sample = effect.process(sample);
        }
        sample
    }
}
