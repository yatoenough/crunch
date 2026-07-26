use std::collections::VecDeque;

pub struct AudioBuffer {
    inner: VecDeque<f32>,
    capacity: usize,
}

impl AudioBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, sample: f32) {
        if self.inner.len() < self.capacity {
            self.inner.push_back(sample);
        }
    }

    pub fn pop(&mut self) -> f32 {
        self.inner.pop_front().unwrap_or(0.0)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new(4096)
    }
}
