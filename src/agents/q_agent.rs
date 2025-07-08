use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::Path, vec};

use crate::{Action, Agent, Environment, Space, SpaceElem};

/// The Agent struct represents the agent that is going to interact and learn from the environment.
/// It contains methods for learning and acting with the environment and useful utils such as loading and saving Q-tables.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QAgent {
    /// Q-table is a 3D array where containing the Q-values for each state-action pair.
    pub q_table: Vec<f32>,
    /// Epsilon-greedy parameters for exploration vs exploitation ε where (0 ≤ ε ≤ 1)
    /// A higher epsilon means more exploration, while a lower epsilon means more exploitation.
    epsilon: f32,
    /// Learning rate α where (0 < α ≤ 1)
    /// A higher alpha means the agent learns more quickly from new information.
    alpha: f32,
    /// Discount factor 𝛾 for future rewards where (0 ≤ γ < 1)
    /// A higher gamma means the agent values future rewards more.
    gamma: f32,
    /// State space
    state_space: Vec<usize>,
    /// State space size
    state_space_size: usize,
    /// Action space
    action_space: Vec<usize>,
    /// Action space size
    action_space_size: usize,
}

const EPSILON_DEFAULT: f32 = 0.05;
const ALPHA_DEFAULT: f32 = 0.1;
const GAMMA_DEFAULT: f32 = 0.9;

impl Default for QAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl QAgent {
    /// Creates a new Agent with an initialized Q-table and parameters for learning.
    pub fn new() -> Self {
        QAgent {
            // Q-table initialized with zeros
            q_table: vec![],
            epsilon: EPSILON_DEFAULT,
            alpha: ALPHA_DEFAULT,
            gamma: GAMMA_DEFAULT,
            state_space: Vec::new(),
            state_space_size: 0,
            action_space: Vec::new(),
            action_space_size: 0,
        }
    }

    pub fn save_to_file(&self, file_path: impl AsRef<Path>) -> std::io::Result<()> {
        create_dir_all(file_path.as_ref().parent().unwrap())?;
        let mut file = std::fs::File::create(file_path)?;
        serde_json::to_writer(&mut file, &self)?;
        Ok(())
    }

    pub fn load_from_file(file_path: impl AsRef<Path>) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let file = std::fs::File::open(file_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    fn space_elem_as_int<El: SpaceElem>(elem: &El, state_space: &[usize]) -> usize {
        let mut state_i = 0;
        for (d, size) in state_space.iter().enumerate() {
            state_i = state_i * size + elem.discrete(d).unwrap();
        }
        state_i
    }

    fn q_val_mut(&mut self, state: &impl SpaceElem, action: &impl Action) -> &mut f32 {
        let state_i = Self::space_elem_as_int(state, &self.state_space);
        let action_i = Self::space_elem_as_int(action, &self.action_space);
        &mut self.q_table[state_i * self.action_space_size + action_i]
    }

    fn q_val(&self, state: &impl SpaceElem, action: &impl Action) -> f32 {
        let state_i = Self::space_elem_as_int(state, &self.state_space);
        let action_i = Self::space_elem_as_int(action, &self.action_space);
        self.q_table[state_i * self.action_space_size + action_i]
    }

    pub fn predict_all<E: Environment>(&self) -> Vec<(E::State, E::Action)> {
        let mut predictions = vec![];
        for state in all_elems_as_vec(&self.state_space) {
            let state = E::State::try_build(&self.state_space.as_slice(), &state, &[]).unwrap();
            let prediction = <QAgent as Agent<E>>::predict(&self, &state);
            predictions.push((state, prediction));
        }
        predictions
    }

    pub fn serialize_q_table(&self) -> Vec<&[f32]> {
        self.q_table
            .chunks(self.state_space_size)
            .collect::<Vec<&[f32]>>()
    }
}

fn all_elems_as_vec(space: &[usize]) -> impl Iterator<Item = Vec<usize>> + '_ {
    let mut indices = vec![0; space.len()];
    let max_indices: Vec<usize> = space.iter().map(|&s| s - 1).collect();
    std::iter::once(vec![0; space.len()]).chain(std::iter::from_fn(move || {
        for i in (0..indices.len()).rev() {
            if indices[i] < max_indices[i] {
                indices[i] += 1;
                return Some(indices.clone());
            }
            indices[i] = 0;
        }
        None
    }))
}

#[test]
fn test_all_elems_as_vec() {
    let space = vec![3, 3];
    let v = all_elems_as_vec(&space).collect::<Vec<_>>();
    assert_eq!(v.len(), 9);
}

fn all_actions<'a, A: Action + 'a>(action_space: &'a[usize]) -> impl Iterator<Item = A> + 'a {
    let mut indices = vec![0; action_space.len()];
    let max_indices: Vec<usize> = action_space.iter().map(|&s| s - 1).collect();
    std::iter::once(A::try_build(&action_space, &indices, &[]).unwrap()).chain(std::iter::from_fn(move || {
        for i in (0..indices.len()).rev() {
            if indices[i] < max_indices[i] {
                indices[i] += 1;
                return Some(A::try_build(&action_space, &indices, &[]).unwrap());
            }
            indices[i] = 0;
        }
        None
    }))
}

impl<E: Environment> Agent<E> for QAgent {
    fn try_init(&mut self, env: &E) -> bool {
        if env.state_space().continuous_dim(0).is_some()
            || env.action_space().continuous_dim(0).is_some()
        {
            // Q-learning does not support continuous spaces
            return false;
        }
        let mut d = 0;
        let mut state_space_size = 1;
        while let Some(size) = env.state_space().discrete_dim(d) {
            state_space_size *= size;
            self.state_space.push(size);
            d += 1;
        }
        self.state_space_size = state_space_size;
        let mut action_space_size = 1;
        let mut d = 0;
        while let Some(size) = env.action_space().discrete_dim(d) {
            action_space_size *= size;
            self.action_space.push(size);
            d += 1;
        }
        self.action_space_size = action_space_size;
        self.q_table = vec![0.0; state_space_size * action_space_size];
        true
    }

    fn act(&mut self, state: &E::State) -> E::Action {
        let mut rng = rand::rng();
        if rng.random::<f32>() < self.epsilon {
            // Exploration: choose a random action
            E::Action::gen_random(&&*self.action_space).unwrap()
        } else {
            // Exploitation: choose the best action based on Q-values
            let mut best_action = E::Action::default();
            let mut best_value = f32::MIN;
            for action in all_actions::<E::Action>(&self.action_space) {
                let q_value: f32 = self.q_val(state, &action);
                if q_value > best_value {
                    best_value = q_value;
                    best_action = action;
                }
            }
            best_action
        }
    }

    /// Applies the **Q‑learning update** to the Q‑table.
    ///
    /// Given a state `s`, action `a`, reward `r`, and next state `s'`, the Q‑value update is computed as:
    ///
    /// ```math
    /// Q(s, a) ← Q(s, a) + α · (r + γ · maxₐ' Q(s', a') − Q(s, a))
    /// ```
    ///
    /// where:
    /// - **α** is the learning rate,
    /// - **γ** is the discount factor.
    ///
    /// **Parameters:**
    ///
    /// - `state`: The current state, as a tuple `(usize, usize)`.
    /// - `action`: The action taken.
    /// - `reward`: The immediate reward received.
    /// - `next_state`: The state resulting after taking the action.
    fn learn(&mut self, state: &E::State, action: &E::Action, reward: f32, next_state: &E::State) {
        let mut max_q_next = f32::MIN;
        for a in all_actions::<E::Action>(&self.action_space) {
            let q_val = self.q_val(next_state, &a);
            if q_val > max_q_next {
                max_q_next = q_val;
            }
        }

        *self.q_val_mut(state, action) +=
            self.alpha * (reward + self.gamma * max_q_next - self.q_val(state, action));
    }

    fn predict(&self, state: &E::State) -> E::Action {
        let mut best_action = E::Action::default();
        let mut best_value = f32::MIN;
        for a in all_actions::<E::Action>(&self.action_space) {
            let q_value: f32 = self.q_val(state, &a);
            if q_value > best_value {
                best_value = q_value;
                best_action = a;
            }
        }
        best_action
    }
}
