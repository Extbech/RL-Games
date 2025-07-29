use crate::{
    agents::{
        network::{
            memory_buffer::{Experience, MemoryBuffer},
            nn::{ActivationFunction, LossFunction, NeuralNetwork},
        },
        q_agent::{all_actions, ALPHA_DEFAULT, EPSILON_DEFAULT, GAMMA_DEFAULT},
    },
    Action, Agent, Environment, Space, SpaceElem,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DQNAgent {
    pub policy_net: NeuralNetwork,
    pub target_net: NeuralNetwork,
    pub memory_buffer: MemoryBuffer,
    pub batch_size: usize,
    pub epsilon: f32,
    pub gamma: f32,
    pub disc_state_space: Vec<usize>,
    pub cont_state_space: Vec<Range<f32>>,
    pub action_space: Vec<usize>,
}

impl DQNAgent {
    pub fn new(buffer_capacity: usize) -> Self {
        DQNAgent {
            policy_net: NeuralNetwork::new(
                ALPHA_DEFAULT as f64,
                ActivationFunction::Sigmoid,
                ActivationFunction::Sigmoid,
                LossFunction::MeanSquaredError,
            ),
            target_net: NeuralNetwork::new(
                ALPHA_DEFAULT as f64,
                ActivationFunction::Sigmoid,
                ActivationFunction::Sigmoid,
                LossFunction::MeanSquaredError,
            ),
            batch_size: 64,
            memory_buffer: MemoryBuffer::new(buffer_capacity),
            epsilon: EPSILON_DEFAULT,
            gamma: GAMMA_DEFAULT,
            disc_state_space: Vec::new(),
            cont_state_space: Vec::new(),
            action_space: Vec::new(),
        }
    }

    fn encode_input(&self, state: &impl SpaceElem) -> Vec<f64> {
        let mut input = vec![];
        for d in 0..self.disc_state_space.len() {
            input.push(state.discrete(d).unwrap() as f64 / self.disc_state_space[d] as f64);
        }
        for d in 0..self.cont_state_space.len() {
            let offset = self.cont_state_space[d].start as f64;
            let range = self.cont_state_space[d].end - self.cont_state_space[d].start;
            input.push((state.continuous(d).unwrap() as f64 - offset) / range as f64);
        }
        input
    }

    fn predict_network(&self, state: &impl SpaceElem) -> Vec<f64> {
        let input = self.encode_input(state);
        self.policy_net.predict(input)
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, String> {
        let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.to_string())
    }

    pub fn save_to_file(&mut self, file_path: &str) -> Result<(), String> {
        let file = std::fs::File::create(file_path).map_err(|e| e.to_string())?;
        self.memory_buffer = MemoryBuffer::new(0);
        serde_json::to_writer(file, self).map_err(|e| e.to_string())
    }
}

impl<E: Environment> Agent<E> for DQNAgent {
    fn try_init(&mut self, env: &E) -> bool {
        let cont_action;
        (self.action_space, cont_action) = env.action_space().as_vecs();
        if !cont_action.is_empty() {
            return false; // DQN does not support continuous action spaces
        }
        (self.disc_state_space, self.cont_state_space) = env.state_space().as_vecs();
        let input_dims = self.disc_state_space.len() + self.cont_state_space.len();
        let output_dims = self.action_space.iter().product();
        // First layer has size of total dimensions of state and action spaces
        // Last Layer has only one ouput, the Q-value
        self.policy_net.add_layers(&[input_dims, 64, output_dims]);
        self.target_net.add_layers(&[input_dims, 64, output_dims]);
        true
    }

    fn act(&mut self, state: &<E as Environment>::State) -> <E as Environment>::Action {
        let mut rng = rand::rng();
        if rng.random::<f32>() < self.epsilon {
            // Exploration: choose a random action
            E::Action::gen_random(&&*self.action_space).unwrap()
        } else {
            <Self as Agent<E>>::predict(&self, state)
        }
    }

    fn learn(
        &mut self,
        old_state: &<E as Environment>::State,
        action: &<E as Environment>::Action,
        reward: f32,
        next_state: Option<&<E as Environment>::State>,
    ) {
        // Add experience to memory buffer
        self.memory_buffer.add_experience(Experience::new(
            self.encode_input(old_state),
            action.discrete(0).unwrap(),
            reward,
            self.encode_input(next_state.unwrap()),
            next_state.is_none(),
        ));
        // If the memory buffer is not full enough, we cannot learn yet
        if self.memory_buffer.buffer.len() < self.batch_size {
            return;
        }
        // Sample a batch of experiences from the memory buffer
        todo!("Implement DQN learning algorithm");
        // let (state_batch, action_batch, reward_batch, next_state_bach, done_batch) =
        //     self.memory_buffer.sample_and_unpack(self.batch_size);
    }

    fn predict(&self, state: &<E as Environment>::State) -> <E as Environment>::Action {
        // Exploitation: choose the best action based on Q-values
        let action = self
            .predict_network(state)
            .iter()
            .enumerate()
            .max_by(|(_i, x), (_j, y)| x.partial_cmp(y).unwrap())
            .unwrap()
            .0;
        all_actions(&self.action_space).take(action).next().unwrap()
    }
}
