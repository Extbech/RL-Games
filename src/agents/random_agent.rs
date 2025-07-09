use crate::{Agent, Action, Environment};

pub struct RandomAgent<E: Environment> {
    action_space: <E as Environment>::ActionSpace,
}

impl<E: Environment> RandomAgent<E> {
    pub fn new() -> Self {
        Self {action_space: <E as Environment>::ActionSpace::default()}
    }
}

impl<E: Environment> Agent<E> for RandomAgent<E> {
    fn try_init(&mut self, _env: &E) -> bool {
        true
    }

    fn predict(&self, _state: &<E as Environment>::State) -> <E as Environment>::Action {
        // Randomly select an action from the action space
        <E as Environment>::Action::gen_random(&self.action_space).unwrap()
    }
}