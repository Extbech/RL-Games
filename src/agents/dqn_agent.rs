use std::ops::Range;
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use crate::{agents::{network::nn::{ActivationFunction, LossFunction, NeuralNetwork}, q_agent::{all_actions, ALPHA_DEFAULT, EPSILON_DEFAULT, GAMMA_DEFAULT}}, Action, Agent, Environment, Space, SpaceElem};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DQNAgent {
    pub network: NeuralNetwork,
    pub epsilon: f32,
    pub gamma: f32,
    pub disc_state_space: Vec<usize>,
    pub cont_state_space: Vec<Range<f32>>,
    pub action_space: Vec<usize>,
}

impl DQNAgent {
    pub fn new() -> Self {
        DQNAgent {
            network: NeuralNetwork::new(
                ALPHA_DEFAULT as f64,
                ActivationFunction::Sigmoid,
                ActivationFunction::Sigmoid,
                LossFunction::MeanSquaredError
            ),
            epsilon: EPSILON_DEFAULT,
            gamma: GAMMA_DEFAULT,
            disc_state_space: Vec::new(),
            cont_state_space: Vec::new(),
            action_space: Vec::new(),
        }
    }

    fn encode_input(&self, state: &impl SpaceElem, action: &impl Action) -> Vec<f64> {
        let mut input = vec![];
        for d in 0..self.disc_state_space.len() {
            input.push(state.discrete(d).unwrap() as f64 / self.disc_state_space[d] as f64);
        }
        for d in 0..self.cont_state_space.len() {
            let offset = self.cont_state_space[d].start as f64;
            let range = self.cont_state_space[d].end - self.cont_state_space[d].start;
            input.push((state.continuous(d).unwrap() as f64 - offset) / range as f64);
        }
        for d in 0..self.action_space.len() {
            input.push(action.discrete(d).unwrap() as f64 / self.action_space[d] as f64);
        }
        input
    }

    fn q_val(&self, state: &impl SpaceElem, action: &impl Action) -> f64 {
        let input = self.encode_input(state, action);
        self.network.predict(input)[0]
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, String> {
        let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.to_string())
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let file = std::fs::File::create(file_path).map_err(|e| e.to_string())?;
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
        let tot_dims = self.disc_state_space.len() + self.cont_state_space.len() +
            self.action_space.len();
        // First layer has size of total dimensions of state and action spaces
        // Last Layer has only one ouput, the Q-value
        self.network.add_layers(&[tot_dims, 64, 1]);
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
        let input = self.encode_input(old_state, action);
        let mut max_q_next = f64::MIN;
        if let Some(next_state) = next_state {
            for a in all_actions::<E::Action>(&self.action_space) {
                let q_val = self.q_val(next_state, &a);
                if q_val > max_q_next {
                    max_q_next = q_val;
                }
            }
        } else {
            // If there is no next state it is terminal, so we set max_q_next to 0
            max_q_next = 0.0;
        }
        self.network.train(vec![input], vec![vec![self.gamma as f64 * max_q_next + reward as f64]]);
    }

    fn predict(&self, state: &<E as Environment>::State) -> <E as Environment>::Action {
        // Exploitation: choose the best action based on Q-values
        let mut best_action = E::Action::default();
        let mut best_value = f32::MIN;
        for action in all_actions::<E::Action>(&self.action_space) {
            let q_value: f32 = self.q_val(state, &action) as f32;
            if q_value > best_value {
                best_value = q_value;
                best_action = action;
            }
        }
        best_action
    }
}
