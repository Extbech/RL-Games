use crate::{Space, SpaceElem, State, StateSpace};
use serde::{Deserialize, Serialize};

use crate::{Action, Environment, Step};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
pub struct TicTacAction(usize, usize);

impl SpaceElem for TicTacAction {
    fn discrete(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(self.0),
            1 => Some(self.1),
            _ => None,
        }
    }

    fn continuous(&self, _d: usize) -> Option<f32> {
        None
    }

    fn try_build(_: &impl Space, discrete: &[usize], continuous: &[f32]) -> Option<Self> {
        if discrete.len() == 2 && continuous.is_empty() {
            Some(Self(discrete[0], discrete[1]))
        } else {
            None
        }
    }
}

impl Action for TicTacAction {
    fn is_valid(&self, space: &impl Space) -> bool {
        space.discrete_dim(0).is_some()
            && space.discrete_dim(1).is_some()
            && self.0 < space.discrete_dim(0).unwrap()
            && self.1 < space.discrete_dim(1).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CellState {
    Empty,
    X,
    O,
}

impl CellState {
    /// Converts a usize to an CellState enum.
    pub fn from_usize(action: usize) -> Self {
        match action {
            0 => CellState::Empty,
            1 => CellState::X,
            2 => CellState::O,
            _ => panic!("Invalid action"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Board {
    pub cells: [[CellState; 3]; 3],
    pub player: TicTacPlayer,
    pub done: bool,
}

impl SpaceElem for Board {
    fn discrete(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(self.cells[0][0] as usize),
            1 => Some(self.cells[0][1] as usize),
            2 => Some(self.cells[0][2] as usize),
            3 => Some(self.cells[1][0] as usize),
            4 => Some(self.cells[1][1] as usize),
            5 => Some(self.cells[1][2] as usize),
            6 => Some(self.cells[2][0] as usize),
            7 => Some(self.cells[2][1] as usize),
            8 => Some(self.cells[2][2] as usize),
            9 => Some(self.done as usize),
            _ => None,
        }
    }

    fn continuous(&self, _d: usize) -> Option<f32> {
        None
    }

    fn try_build(_: &impl Space, discrete: &[usize], continuous: &[f32]) -> Option<Self> {
        if discrete.len() == 10 && continuous.is_empty() {
            let x_cells = discrete.iter().filter(|&&x| x == 1).count();
            let o_cells = discrete.iter().filter(|&&x| x == 2).count();
            let player = match x_cells as isize - o_cells as isize {
                0 => TicTacPlayer::X, // X starts first
                1 => TicTacPlayer::O, // O's turn
                _ => return None,     // Invalid state
            };
            let mut temp = Self {
                cells: [[CellState::Empty; 3]; 3],
                player,
                done: discrete[9] == 1,
            };
            for n in discrete.iter().enumerate() {
                let row = n.0 / 3;
                let col = n.0 % 3;
                temp.cells[row][col] = CellState::from_usize(*n.1);
            }
            Some(temp)
        } else {
            None
        }
    }
}

impl State for Board {
    fn current_player(&self) -> usize {
        match self.player {
            TicTacPlayer::X => 0,
            TicTacPlayer::O => 1,
        }
    }
}

#[derive(Default, Clone)]
pub struct Shape;

impl Space for Shape {
    fn discrete_dim(&self, d: usize) -> Option<usize> {
        if d < 9 {
            Some(3)
        } else if d == 9 {
            Some(2) // For the done state
        } else {
            None
        }
    }

    fn continuous_dim(&self, _d: usize) -> Option<std::ops::Range<f32>> {
        None
    }
}

impl StateSpace for Shape {
    fn player_count(&self) -> usize {
        2 // Tic Tac Toe always has two players: X and O
    }
}

#[derive(Default, Clone)]
pub struct TicTacActionSpace;

impl Space for TicTacActionSpace {
    fn discrete_dim(&self, d: usize) -> Option<usize> {
        match d {
            0 => Some(3), // Rows
            1 => Some(3), // Columns
            _ => None,
        }
    }

    fn continuous_dim(&self, _d: usize) -> Option<std::ops::Range<f32>> {
        None
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TicTacPlayer {
    X,
    O,
}

pub struct TicTacEnvironment {
    pub board: Board,
    pub reward: [f32; 2],
}

impl TicTacEnvironment {
    pub fn new() -> Self {
        TicTacEnvironment {
            board: Board {
                cells: [[CellState::Empty; 3]; 3],
                player: TicTacPlayer::X, // X starts first
                done: false,
            },
            reward: [0.0, 0.0],
        }
    }

    fn calc_reward(&mut self) {
        // Check rows, columns, and diagonals for a win
        for i in 0..3 {
            if self.board.cells[i][0] == self.board.cells[i][1]
                && self.board.cells[i][1] == self.board.cells[i][2]
            {
                match self.board.cells[i][0] {
                    CellState::X => {
                        self.reward = [1.0, -1.0]; // X wins
                        self.board.done = true;
                    }
                    CellState::O => {
                        self.reward = [-1.0, 1.0]; // O wins
                        self.board.done = true;
                    }
                    CellState::Empty => continue, // No winner yet
                }
            }
            if self.board.cells[0][i] == self.board.cells[1][i]
                && self.board.cells[1][i] == self.board.cells[2][i]
            {
                match self.board.cells[0][i] {
                    CellState::X => {
                        self.reward = [1.0, -1.0]; // X wins
                        self.board.done = true;
                    }
                    CellState::O => {
                        self.reward = [-1.0, 1.0]; // O wins
                        self.board.done = true;
                    }
                    CellState::Empty => continue, // No winner yet
                }
            }
        }
        if (self.board.cells[0][0] == self.board.cells[1][1]
            && self.board.cells[1][1] == self.board.cells[2][2])
            || (self.board.cells[0][2] == self.board.cells[1][1]
                && self.board.cells[1][1] == self.board.cells[2][0])
        {
            match self.board.cells[1][1] {
                CellState::X => {
                    self.reward = [1.0, -1.0]; // X wins
                    self.board.done = true;
                }
                CellState::O => {
                    self.reward = [-1.0, 1.0]; // O wins
                    self.board.done = true;
                }
                CellState::Empty => return, // No winner yet
            }
        }
    }
    fn is_draw(&self) -> bool {
        self.board
            .cells
            .iter()
            .all(|row| row.iter().all(|&cell| cell != CellState::Empty))
    }
}

impl Environment for TicTacEnvironment {
    type Action = TicTacAction;
    type State = Board;
    type StateSpace = Shape;
    type ActionSpace = TicTacActionSpace;

    fn action_space(&self) -> &Self::ActionSpace {
        &TicTacActionSpace
    }

    fn state_space(&self) -> &Self::StateSpace {
        &Shape
    }

    /// Resets the environment and sets a new random starting position so that our agent does not always start in the top-left corner.
    fn reset(&mut self) -> &Self::State {
        *self = TicTacEnvironment::new();
        &self.board
    }

    /// Steps through the environment based on the action taken by the agent.
    /// It updates the agent's position, calculates the reward, and checks if the game is finished.
    fn step(&mut self, action: &Self::Action) -> Step<Self> {
        match self.board.player {
            TicTacPlayer::X => {
                if self.board.cells[action.0][action.1] == CellState::Empty {
                    self.board.cells[action.0][action.1] = CellState::X;
                    self.calc_reward();
                    self.board.player = TicTacPlayer::O;
                } else {
                    if self.is_draw() {
                        self.reward = [0.0, 0.0]; // Draw
                        self.board.done = true;
                    } else {
                        self.reward = [-100.0, 1.0]; // Invalid move
                        self.board.done = true;
                    }
                }
            }
            TicTacPlayer::O => {
                if self.board.cells[action.0][action.1] == CellState::Empty {
                    self.board.cells[action.0][action.1] = CellState::O;
                    self.calc_reward();
                    self.board.player = TicTacPlayer::X;
                } else {
                    if self.is_draw() {
                        self.reward = [0.0, 0.0]; // Draw
                        self.board.done = true;
                    } else {
                        self.reward = [1.0, -100.0]; // Invalid move
                        self.board.done = true;
                    }
                }
            }
        }
        if self.board.done {
            Step {
                reward: &self.reward,
                next_state: None, // No next state if the game is done
            }
        } else {
            // If the game is not done, return the current state
            Step {
                reward: &self.reward,
                next_state: Some(&self.board),
            }
        }
    }
}
