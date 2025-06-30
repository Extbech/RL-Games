use crate::{Space, SpaceElem};
use rand::{rng, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{Action, Environment, Step};

pub const GRID_SIZE: (usize, usize) = (9, 9);

/// The Action enum represents the possible actions the agent can take in the environment.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for MoveAction {
    fn default() -> Self {
        Self::Up
    }
}

impl SpaceElem for MoveAction {
    fn discrete(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(match self {
                MoveAction::Up => 0,
                MoveAction::Down => 1,
                MoveAction::Left => 2,
                MoveAction::Right => 3,
            }),
            _ => None,
        }
    }

    fn continuous(&self, _d: usize) -> Option<f32> {
        None
    }
}

impl Action for MoveAction {
    fn try_build(
        _space: &impl Space,
        discrete: &[usize],
        _continuous: &[f32],
    ) -> Option<Self> {
        if discrete.len() == 1 {
            match discrete[0] {
                0 => Some(Self::Up),
                1 => Some(Self::Down),
                2 => Some(Self::Left),
                3 => Some(Self::Right),
                _ => None,
            }
        } else {
            None
        }
    }

    fn is_valid(&self, space: &impl Space) -> bool {
        space.discrete_dim(0).is_some() && space.discrete_dim(0) == Some(4)
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    position: (usize, usize),
}

impl SpaceElem for Board {
    fn discrete(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(self.position.0),
            1 => Some(self.position.1),
            _ => None,
        }
    }

    fn continuous(&self, _d: usize) -> Option<f32> {
        None
    }
}

#[derive(Default, Clone)]
pub struct Shape {
    rows: usize,
    cols: usize,
}

impl Space for Shape {
    fn discrete_dim(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(self.rows),
            1 => Some(self.cols),
            _ => None,
        }
    }

    fn continuous_dim(&self, _d: usize) -> Option<std::ops::Range<f32>> {
        None
    }
}

#[derive(Default, Clone)]
pub struct MoveActionSpace;

impl Space for MoveActionSpace {
    fn discrete_dim(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(4), // Four possible actions
            _ => None,
        }
    }

    fn continuous_dim(&self, _d: usize) -> Option<std::ops::Range<f32>> {
        None
    }
}

/// The Environment struct represents the environment in which the agent operates.
/// It contains the current position of the agent, the board state which is a `5x5` grid, the reward for the last action, and the current game state.
pub struct GridEnvironment {
    pub shape: Shape,
    pub board: Board,
    // pub walls: Vec<(usize, usize)>,
    pub reward: f32,
    pub done: bool,
}

impl GridEnvironment {
    /// Creates a new Environment with an initial position, empty board, and default values for reward and game state.
    pub fn new(rows: usize, cols: usize) -> Self {
        GridEnvironment {
            board: Board {
                position: (0, 0)
            },
            shape: Shape {
                rows,
                cols,
            },
            reward: 0.0,
            done: false,
        }
    }

    /// Calculates the reward based on the agent's current position.
    /// If the agent reaches the center of the board, it receives a reward of 100.0 and the game ends.
    /// Otherwise, it calculates the reward based on the Euclidean distance from the center of the board.
    /// Using the formula `r = 1 / √(x2 – x1)^2 + (y2 – y1)^2`
    fn calc_reward(&mut self) {

        if self.board.position.0 == self.shape.cols / 2
            && self.board.position.1 == self.shape.rows / 2
        {
            self.reward = 100.0;
            self.done = true
        } else {
            self.reward = 1.
                / f32::sqrt(
                    (self.board.position.0 as f32
                        - (self.shape.cols / 2) as f32)
                        .powi(2)
                        + (self.board.position.1 as f32
                            - (self.shape.rows / 2) as f32)
                            .powi(2),
                );
        }
    }
}

impl Environment for GridEnvironment {
    type Action = MoveAction;
    type State = Board;
    type StateSpace = Shape;
    type ActionSpace = MoveActionSpace;

    fn action_space(&self) -> &Self::ActionSpace {
        &MoveActionSpace
    }

    fn state_space(&self) -> &Self::StateSpace {
        &self.shape
    }

    /// Resets the environment and sets a new random starting position so that our agent does not always start in the top-left corner.
    fn reset(&mut self) -> &Self::State {

        self.reward = 0.0;
        self.done = false;
        self.board.position = (0, 0);
        while self.board.position.0 == 0
            && self.board.position.1 == 0
        {
            self.board.position = (
                rng().random_range(0..self.shape.cols),
                rng().random_range(0..self.shape.rows),
            );
        }
        &self.board
    }

    /// Steps through the environment based on the action taken by the agent.
    /// It updates the agent's position, calculates the reward, and checks if the game is finished.
    fn step(&mut self, action: &Self::Action) -> Step<Self> {
        match action {
            MoveAction::Up => {
                if self.board.position.0 > 0 {
                    self.board.position.0 -= 1;
                    self.calc_reward()
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Down => {
                if self.board.position.0 < self.shape.rows - 1 {
                    self.board.position.0 += 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Left => {
                if self.board.position.1 > 0 {
                    self.board.position.1 -= 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
            MoveAction::Right => {
                if self.board.position.1 < self.shape.cols - 1 {
                    self.board.position.1 += 1;
                    self.calc_reward();
                } else {
                    self.calc_reward();
                    self.done = true;
                }
            }
        }
        Step {
            is_final: self.done,
            reward: self.reward,
            next_state: &self.board,
        }
    }
}
