use std::fmt::Write;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::{Agent, Environment, Step};

/// Trains the agent by running a specified number of episodes in the environment.
/// Each episode consists of the agent taking actions in the environment until a terminal state is reached. e.g. the agent either won or lost.
pub fn train<E: Environment, A: Agent<E>>(env: &mut E, agent: &mut A, episodes: u64) {
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
        loop {
            let action = agent.act(&state);
            let Step {
                is_final,
                reward,
                next_state,
            } = env.step(&action);
            // Update the Q-table using Q-learning update
            agent.learn(&state, &action, reward, next_state);
            if is_final {
                break;
            }

            // Set the next state as current for the following iteration
            state = next_state.clone();
        }
        pb.set_position(episode);
    }
}
