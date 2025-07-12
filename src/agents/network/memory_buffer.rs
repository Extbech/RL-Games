use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    state: Vec<f64>,
    action: usize,
    reward: f32,
    next_state: Vec<f64>,
    done: bool,
}

impl Experience {
    pub fn new(
        state: Vec<f64>,
        action: usize,
        reward: f32,
        next_state: Vec<f64>,
        done: bool,
    ) -> Self {
        Experience {
            state,
            action,
            reward,
            next_state,
            done,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBuffer {
    pub buffer: VecDeque<Experience>,
    pub capacity: usize,
}

impl MemoryBuffer {
    pub fn new(capacity: usize) -> Self {
        MemoryBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_experience(&mut self, experience: Experience) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(experience);
    }

    pub fn sample(&self, batch_size: usize) -> Vec<&Experience> {
        let mut rng = rand::rng();
        let indices: Vec<usize> = (0..self.buffer.len()).collect();
        let sampled_indices = indices
            .choose_multiple(&mut rng, batch_size)
            .collect::<Vec<_>>();
        sampled_indices.iter().map(|&i| &self.buffer[*i]).collect()
    }

    pub fn sample_and_unpack(
        &self,
        batch_size: usize,
    ) -> (
        Vec<Vec<f64>>,
        Vec<usize>,
        Vec<f32>,
        Vec<Vec<f64>>,
        Vec<bool>,
    ) {
        let sample = self.sample(batch_size);
        let (states, actions, rewards, next_states, dones) = sample
            .into_iter()
            .map(|exp| {
                (
                    exp.state.clone(),
                    exp.action,
                    exp.reward,
                    exp.next_state.clone(),
                    exp.done,
                )
            })
            .collect();
        (states, actions, rewards, next_states, dones)
    }
}
