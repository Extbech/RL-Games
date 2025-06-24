use rand::Rng;
use std::{
    collections::HashMap, io::{BufRead, BufReader, Write}, vec
};
use ordered_float::NotNan;

use crate::{environment::move_to_center::{GridEnvironment, MoveAction}, Action, Agent, Environment};

/// The Agent struct represents the agent that is going to interact and learn from the environment.
/// It contains methods for learning and acting with the environment and useful utils such as loading and saving Q-tables.
pub struct QAgent<E: Environment> {
    /// Q-table is a 3D array where containing the Q-values for each state-action pair.
    pub q_table: HashMap::<(Vec<NotNan<f32>>, E::Action), f32>,
    /// Epsilon-greedy parameters for exploration vs exploitation ε where (0 ≤ ε ≤ 1)
    /// A higher epsilon means more exploration, while a lower epsilon means more exploitation.
    epsilon: f32,
    /// Learning rate α where (0 < α ≤ 1)
    /// A higher alpha means the agent learns more quickly from new information.
    alpha: f32,
    /// Discount factor 𝛾 for future rewards where (0 ≤ γ < 1)
    /// A higher gamma means the agent values future rewards more.
    gamma: f32,
}

/// TODO: Ask the bastard who wrote this what it does.
fn kfc(fv: Vec<f32>) -> Vec<NotNan<f32>> {
    fv.into_iter().map(|f| NotNan::new(f).unwrap()).collect()
}

impl<E: Environment> QAgent<E> {
    /// Creates a new Agent with an initialized Q-table and parameters for learning.
    pub fn new() -> Self {
        QAgent::<E> {
            // Q-table initialized with zeros
            q_table: HashMap::new(),
            epsilon: 0.05,
            alpha: 0.1,
            gamma: 0.9,
        }
    }
}
/*
impl QAgent<GridEnvironment> {
    /// Converts the Q-table to a 2D grid of actions, where each cell contains the action with the highest Q-value.
    /// This is useful for visualizing the agent's Q-table in a more human-readable format.
    pub fn q_table_to_2_dim_grid(&self) -> Vec<Vec<MoveAction>> {
        let mut grid = vec![vec![MoveAction::Up; self.q_table[0].len()]; self.q_table.len()];
        for i in 0..self.q_table.len() {
            for j in 0..self.q_table[i].len() {
                grid[i][j] = MoveAction::try_from_usize(
                    self.q_table[i][j]
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.total_cmp(b))
                        .map(|(index, _)| index)
                        .unwrap() as usize,
                ).unwrap();
            }
        }
        grid
    }
} */

impl<E: Environment> Agent<E> for QAgent<E> {
    fn act(&mut self, state: Vec<f32>) -> E::Action {
        let mut rng = rand::rng();
        let action = if rng.random::<f32>() < self.epsilon {
            // Exploration: choose a random action
            E::Action::gen_random_state()
        } else {
            // Exploitation: choose the best action based on Q-values
            let mut best_action = Action::try_from_usize(0).unwrap();
            let mut best_value = f32::MIN;
            for action in E::Action::all_actions() {
                let q_value: f32 = *self.q_table.entry((kfc(state.clone()), action)).or_insert(0.);
                if q_value > best_value {
                    best_value = q_value;
                    best_action = action;
                }
            }
            best_action
        };
        action
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
    fn learn(
        &mut self,
        state: Vec<f32>,
        action: E::Action,
        reward: Option<f32>,
        next_state: Vec<f32>,
    ) {
        let mut max_q_next = 0.0;
        for action in E::Action::all_actions() {
            let q_val: f32 = *self.q_table.entry((kfc(next_state.clone()), action)).or_insert(0.);

            if q_val > max_q_next {
                max_q_next = q_val;
            }
        }

        *self.q_table.get_mut(&(kfc(state.clone()), action)).unwrap() += self.alpha
            * (reward.unwrap_or_default() + self.gamma * max_q_next
                - *self.q_table.entry((kfc(state.clone()), action)).or_default());
    }

    fn predict(&self, state: Vec<f32>) -> E::Action {
        let mut best_action = E::Action::try_from_usize(0).unwrap();
        let mut best_value = f32::MIN;
        for a in E::Action::all_actions() {
            let q_value: f32 = self.q_table[&(kfc(state.clone()), a)];
            if q_value > best_value {
                best_value = q_value;
                best_action = a;
            }
        }
        best_action
    }
}

/*
impl
    fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        writeln!(file, "Up, Down, Left, Right")?;
        for row in self.q_table.iter() {
            for col in row.iter() {
                let line = col
                    .iter()
                    .map(|v| format!("{:.2}", v))
                    .collect::<Vec<String>>()
                    .join(",");
                writeln!(file, "{}", line)?;
            }
        }
        Ok(())
    }

    fn load(path: &str, rows: usize, cols: usize) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut agent = QAgent {
            q_table: vec![vec![vec![0.0; 4]; cols]; rows],
            epsilon: 0.2,
            alpha: 0.1,
            gamma: 0.9,
        };
        reader.lines().enumerate().for_each(|(i, val)| {
            let line = val.unwrap();
            if i > 0 {
                let values: Vec<f32> = line
                    .split(',')
                    .map(|s| s.trim().parse().unwrap_or(0.0))
                    .collect();
                agent.q_table[(i - 1) / rows][(i - 1) % rows] =
                    vec![values[0], values[1], values[2], values[3]];
            }
        });
        Ok(agent)
    }
}*/
