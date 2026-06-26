use std::collections::VecDeque;

pub struct AudioBuffer {
    inner: VecDeque<f32>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn push(&mut self, sample: f32) {
        self.inner.push_back(sample);
    }

    pub fn pop(&mut self) -> f32 {
        self.inner.pop_front().unwrap_or(0.0)
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}
