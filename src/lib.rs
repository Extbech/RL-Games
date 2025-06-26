use std::hash::Hash;

use serde::{Serialize};

pub mod agents;
pub mod environment;
pub mod train;

pub trait Action<'de>: Sized + Hash + Eq + Copy + Serialize + 'static {
    const COUNT: usize;

    fn gen_random_state() -> Self;

    fn try_from_usize(x: usize) -> Option<Self>;

    fn to_usize(&self) -> usize;

    fn all_actions() -> impl Iterator<Item=Self> {
        (0..Self::COUNT).map(|x| Self::try_from_usize(x).unwrap())
    }
}


pub struct Step {
    is_final: bool,
    reward: Option<f32>, // Consider dropping the option
    next_state: Vec<f32>
}

pub trait Environment: Serialize {
    type Action: Action<'static>;

    fn reset(&mut self) -> Vec<f32>;

    fn step(&mut self, action: Self::Action) -> Step;
}


pub trait Agent<E: Environment> {
    /// Returns the action to be taken during learning in the current state.
    ///
    /// # Arguments
    ///
    /// * `state` - A tuple representing the current state.
    fn act(&mut self, state: Vec<f32>) -> E::Action;

    /// Updates the agent's knowledge based on the action taken and the received reward.
    ///
    /// # Arguments
    ///
    /// * `state` - The current state before taking the action.
    /// * `action` - The action taken.
    /// * `reward` - The immediate reward received after taking the action.
    /// * `next_state` - The state after taking the action.
    fn learn(
        &mut self,
        state: Vec<f32>,
        action: E::Action,
        reward: Option<f32>,
        next_state: Vec<f32>,
    );

    /// Returns the most preferred action, as opposed to `act`, which may
    /// do something worse to learn.
    /// # Arguments
    /// * `state` - A tuple representing the current state.
    /// Returns the predicted action.
    fn predict(&self, state: Vec<f32>) -> E::Action;
}
