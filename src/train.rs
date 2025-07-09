use std::{cell::RefCell, fmt::Write, rc::Rc};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::{Agent, Environment, Step, State, StateSpace};

/// Trains the agent by running a specified number of episodes in the environment.
/// Each episode consists of the agent taking actions in the environment until a terminal state is reached. e.g. the agent either won or lost.
pub fn train<E: Environment>(env: &mut E, agents: &[Rc<RefCell<dyn Agent<E>>>], episodes: u64) {
    assert!(env.state_space().player_count() == agents.len(), "Number of agents must match the number of players in the environment.");
    let pb = ProgressBar::new(episodes);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    for episode in 1..=episodes {
        let mut state = env.reset().clone();
        let mut rewards = vec![0.0; agents.len()];
        let mut prev = vec![];
        prev.resize_with(agents.len(), || None);
        let current_player = state.current_player();
        loop {
            // If the current player has done an action before, we can learn from it
            if let Some((prev_state, action)) = &prev[current_player] {
                // If the previous state is not None, we can learn from it
                agents[current_player].borrow_mut().learn(prev_state, action, rewards[current_player], &state);
                rewards[current_player] = 0.0; // Reset the reward for the current player
            }
            // Get the next action from the current player
            let action = agents[current_player].borrow_mut().act(&state);
            let Step {
                is_final,
                reward,
                next_state,
            } = env.step(&action);
            // Update the rewards
            for player in 0..agents.len() {
                rewards[player] += reward[player];
            }
            // Remember the action and the state
            prev[current_player] = Some((state.clone(), action));
            // If we are finished everyone must learn
            if is_final {
                for player in 0..agents.len() {
                    if let Some((prev_state, action)) = &prev[player] {
                        // If the previous state is not None, we can learn from it
                        agents[current_player].borrow_mut().learn(prev_state, action, rewards[current_player], &state);
                    }
                }
                break;
            }

            // Set the next state as current for the following iteration
            state = next_state.clone();
        }
        pb.set_position(episode);
    }
}
