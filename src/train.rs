use std::{cell::RefCell, rc::Rc};

use indicatif::ProgressBar;

use crate::{Agent, Environment, State, StateSpace, Step};

/// Trains the agent by running a specified number of episodes in the environment.
/// Each episode consists of the agent taking actions in the environment until a terminal state is reached. e.g. the agent either won or lost.
pub fn train_q<E: Environment>(
    env: &mut E,
    agents: &[Rc<RefCell<dyn Agent<E>>>],
    episodes: u64,
    pb: ProgressBar,
) {
    assert!(
        env.state_space().player_count() == agents.len(),
        "Number of agents must match the number of players in the environment."
    );

    for episode in 1..=episodes {
        let state = env.reset().clone();
        let mut o_state = Some(state);
        let mut rewards = vec![0.0; agents.len()];
        let mut prev = vec![];
        prev.resize_with(agents.len(), || None);
        while let Some(state) = o_state {
            let current_player = state.current_player();
            // If the current player has done an action before, we can learn from it
            if let Some((prev_state, action)) = &prev[current_player] {
                // If the previous state is not None, we can learn from it
                agents[current_player].borrow_mut().learn(
                    prev_state,
                    action,
                    rewards[current_player],
                    Some(&state),
                );
                rewards[current_player] = 0.0; // Reset the reward for the current player
            }
            // Get the next action from the current player
            let action = agents[current_player].borrow_mut().act(&state);
            let Step { reward, next_state } = env.step(&action);
            // Update the rewards
            for player in 0..agents.len() {
                rewards[player] += reward[player];
            }
            // Remember the action and the state
            prev[current_player] = Some((state.clone(), action));

            // Set the next state as current for the following iteration
            o_state = next_state.cloned();
        }
        for player in 0..agents.len() {
            if let Some((prev_state, action)) = &prev[player] {
                // If the previous state is not None, we can learn from it
                agents[player]
                    .borrow_mut()
                    .learn(prev_state, action, rewards[player], None);
            }
        }
        pb.set_position(episode);
    }
    pb.finish_with_message("Training completed");
}

pub fn train_dqn<E: Environment>(
    env: &mut E,
    agent: &mut dyn Agent<E>,
    episodes: u64,
    pb: ProgressBar,
) {
    assert!(
        env.state_space().player_count() == 1,
        "DQN agent can only be used in single-player environments."
    );

    for episode in 1..=episodes {
        todo!("Implement DQN training logic here");
        pb.set_position(episode);
    }
    pb.finish_with_message("Training completed");
}
