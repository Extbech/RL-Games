use std::fmt::Write;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::{agents::q_agent::QAgent, environment::move_to_center::GridEnvironment, Agent, Environment, Step};

/// The Trainer struct is responsible for managing the training process of the agent in the environment.
pub struct Trainer<E: Environment, A: Agent<E>> {
    pub environment: E,
    pub agent: A,
}

impl<E: Environment, A: Agent<E>> Trainer<E, A> {
    /// Creates a new Trainer with an initialized Agent and Environment.
    pub fn new(environment: E, agent: A) -> Self {
        Self { environment, agent }
    }

    /// Trains the agent by running a specified number of episodes in the environment.
    /// Each episode consists of the agent taking actions in the environment until a terminal state is reached. e.g. the agent either won or lost.
    pub fn train(&mut self, episodes: u64) {
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
            let mut state = self.environment.reset();

            loop {
                let action = self.agent.act(state.clone());
                let Step {is_final, reward, next_state} = self.environment.step(action);

                // Update the Q-table using Q-learning update
                self.agent.learn(state.clone(), action, reward, next_state.clone());

                // Set the next state as current for the following iteration
                state = next_state;

                if is_final {
                    break;
                }
            }
            pb.set_position(episode);
        }
    }
}
