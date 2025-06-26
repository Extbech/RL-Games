use rand::prelude::*;

pub const GRID_SIZE: (usize, usize) = (9, 9);

/// The Action enum represents the possible actions the agent can take in the environment.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}

impl Action<'_> for MoveAction {
    const COUNT: usize = 4;

    /// Returns a random action from the available actions.
    /// This is useful for exploration in reinforcement learning.
    fn gen_random_state() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            _ => Self::Right,
        }
    }
    /// Converts a usize to an `MoveAction` enum.
    fn try_from_usize(action: usize) -> Option<Self> {
        match action {
            0 => Some(Self::Up),
            1 => Some(Self::Down),
            2 => Some(Self::Left),
            3 => Some(Self::Right),
            _ => None,
        }
    }
    /// Converts an Action enum to a usize.
    fn to_usize(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
        }
    }
}

use rand::{rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};

use crate::{Action, Environment, Step};

/// The Environment struct represents the environment in which the agent operates.
/// It contains the current position of the agent, the board state which is a `5x5` grid, the reward for the last action, and the current game state.
pub struct GridEnvironment {
    pub position: (usize, usize),
    pub board: Vec<Vec<f32>>,
    // pub walls: Vec<(usize, usize)>,
    pub reward: f32,
    pub done: bool,
}

impl Serialize for GridEnvironment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = vec![0.0; self.board.len() * self.board[0].len()];
        state[self.position.0 * self.board[0].len() + self.position.1] = 1.0;
        state.serialize(serializer)
    }
}
impl GridEnvironment {
    /// Creates a new Environment with an initial position, empty board, and default values for reward and game state.
    pub fn new(rows: usize, cols: usize) -> Self {
        GridEnvironment {
            position: (0, 0),
            board: vec![vec![0.0; cols]; rows], // Initialize a 5x5 grid
            reward: 0.0,
            done: false,
        }
    }

    /// Calculates the reward based on the agent's current position.
    /// If the agent reaches the center of the board, it receives a reward of 100.0 and the game ends.
    /// Otherwise, it calculates the reward based on the Euclidean distance from the center of the board.
    /// Using the formula `r = 1 / √(x2 – x1)^2 + (y2 – y1)^2`
    fn calc_reward(&mut self) {
        if self.position == (self.board.len() / 2, self.board[0].len() / 2) {
            self.reward = 100.0;
            self.done = true
        } else {
            self.reward = 1.
                / f32::sqrt(
                    (self.position.0 as f32 - (self.board.len() / 2) as f32).powi(2)
                        + (self.position.1 as f32 - (self.board[0].len() / 2) as f32).powi(2),
            );
        }
    }

    fn flatten(&self) -> Vec<f32> {
        // vec![self.position.0 as f32, self.position.1 as f32]
        let mut r = vec![0.0; self.board.len() * self.board[0].len()];
        r[self.position.0*self.board.len() + self.position.1] = 1.0;
        r
    }
}

impl Environment for GridEnvironment {
    type Action = MoveAction;

    /// Resets the environment and sets a new random starting position so that our agent does not always start in the top-left corner.
    fn reset(&mut self) -> Vec<f32> {
        let possible_options: Vec<usize> = (0..self.board.len())
            .filter(|x| *x != self.board.len() / 2)
            .collect();
        self.position = (
            *possible_options.choose(&mut rng()).unwrap(),
            *possible_options.choose(&mut rng()).unwrap(),
        );
        self.reward = 0.0;
        self.done = false;
        self.flatten()
    }

    /// Steps through the environment based on the action taken by the agent.
    /// It updates the agent's position, calculates the reward, and checks if the game is finished.
    fn step(&mut self, action: MoveAction) -> Step {
        match action {
            MoveAction::Up => {
                if self.position.0 > 0 {
                    self.position.0 -= 1;
                    self.calc_reward()
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Down => {
                if self.position.0 < self.board[0].len() - 1 {
                    self.position.0 += 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Left => {
                if self.position.1 > 0 {
                    self.position.1 -= 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Right => {
                if self.position.1 < self.board.len() - 1 {
                    self.position.1 += 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
        }
        Step { is_final: self.done, reward: Some(self.reward), next_state: self.flatten() }
    }

}
