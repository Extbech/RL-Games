use rand::random_range;
use serde::Serialize;
use std::ops::Range;

pub mod agents;
pub mod environment;
pub mod train;

pub const GRID_AGENT_SAVE_FILE_PATH: &str = "data/q_tables/grid.json";
pub const TIC_TAC_TOE_AGENT_SAVE_FILE_PATH: &str = "data/q_tables/tic_tac_toe.json";

/// A Generalization of spaces, both state and action spaces
pub trait Space: Default + Clone {
    /// The size of each discrete dimension.
    /// A space that returns `None` for `d`
    /// should also return `None` for greater values.
    fn discrete_dim(&self, d: usize) -> Option<usize>;

    /// The bounds of each continuous dimension.
    /// A space that returns `None` for `d`
    /// should also return `None` for greater values.
    fn continuous_dim(&self, d: usize) -> Option<Range<f32>>;
}

pub trait SpaceElem: Sized + Serialize {
    /// The value at discrete dimension `d`.
    /// An element that returns `None` for `d`
    /// should also return `None` for greater values.
    fn discrete(&self, d: usize) -> Option<usize>;

    /// The value at continuous dimension `d`
    /// An element that returns `None` for `d`
    /// should also return `None` for greater values.
    fn continuous(&self, d: usize) -> Option<f32>;

    /// Attempts to build an element from the given discrete and continuous values.
    fn try_build(space: &impl Space, discrete: &[usize], continuous: &[f32]) -> Option<Self>;
}

// TODO: Reconsider sized bound if we want to use trait objects
// In that case we should return a Option<Box<Self>> instead of Option<Self>
pub trait Action: SpaceElem + Default {
    fn gen_random(space: &impl Space) -> Option<Self> {
        let mut discrete_dims = vec![];
        let mut d = 0;
        while let Some(dim) = space.discrete_dim(d) {
            discrete_dims.push(random_range(0..dim));
            d += 1;
        }
        let mut continuous_dims = vec![];
        let mut d = 0;
        while let Some(range) = space.continuous_dim(d) {
            continuous_dims.push(random_range(range.start..range.end));
            d += 1;
        }
        let r = Self::try_build(space, &discrete_dims, &continuous_dims);
        #[cfg(debug_assertions)]
        if let Some(ref a) = r {
            if !a.is_valid(space) {
                panic!("Generated action is not valid for the given space");
            }
        }
        r
    }

    fn is_valid(&self, space: &impl Space) -> bool {
        let mut d = 0;
        while let (Some(a), Some(s)) = (self.discrete(d), space.discrete_dim(d)) {
            if a >= s {
                return false;
            }
            d += 1;
        }
        let mut d = 0;
        while let (Some(a), Some(r)) = (self.continuous(d), space.continuous_dim(d)) {
            if !r.contains(&a) {
                return false;
            }
            d += 1;
        }
        true
    }
}

pub struct Step<'a, E: Environment + ?Sized> {
    reward: &'a [f32],
    next_state: Option<&'a E::State>,
}

pub trait StateSpace: Space {
    fn player_count(&self) -> usize;
}

trait State: SpaceElem + Clone {
    fn current_player(&self) -> usize;
}

pub trait Environment {
    type StateSpace: StateSpace;
    type ActionSpace: Space;
    type State: State;
    type Action: Action;

    fn state_space(&self) -> &Self::StateSpace;
    fn action_space(&self) -> &Self::ActionSpace;

    /// Resets environment, returning the initial state
    fn reset(&mut self) -> &Self::State;

    /// Uses the given action to perform a step
    fn step<'a>(&'a mut self, action: &Self::Action) -> Step<'a, Self>;
}

pub trait Agent<E: Environment> {
    /// Reads spaces and initializes agent
    /// Returns false if the agent does not support the given spaces.
    fn try_init(&mut self, env: &E) -> bool;

    /// Returns the action to be taken during learning in the current state.
    ///
    /// # Arguments
    ///
    /// * `state` - A tuple representing the current state.
    fn act(&mut self, state: &E::State) -> E::Action {
        self.predict(state)
    }

    /// Updates the agent's knowledge based on the action taken and the received reward.
    ///
    /// # Arguments
    ///
    /// * `old_state` - The state before taking the action.
    /// * `action` - The action taken.
    /// * `reward` - The immediate reward received after taking the action.
    /// * `next_state` - The state after taking the action.
    #[allow(unused_variables)]
    fn learn(
        &mut self,
        old_state: &E::State,
        action: &E::Action,
        reward: f32,
        next_state: Option<&E::State>,
    ) {
    }

    /// Selects the most preferred action, as opposed to `act`, which may
    /// do something worse to learn.
    /// # Arguments
    /// * `state` - A tuple representing the current state.
    /// * `action` - An out parameter for the action to be taken.
    fn predict(&self, state: &E::State) -> E::Action;
}

impl Space for &[usize] {
    fn discrete_dim(&self, d: usize) -> Option<usize> {
        self.get(d).cloned()
    }

    fn continuous_dim(&self, _d: usize) -> Option<std::ops::Range<f32>> {
        None
    }
}
