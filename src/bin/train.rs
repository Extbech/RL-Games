use std::time::Instant;

use rust_rl::{agents::q_agent::QAgent, environment::move_to_center::GridEnvironment, train::Trainer, Agent};

const GRID_SIZE: (usize, usize) = (9, 9);

fn main() {
    let start = Instant::now();
    let env = GridEnvironment::new(GRID_SIZE.0, GRID_SIZE.1);
    let agent = QAgent::new();
    let mut trainer = Trainer::new(env, agent);
    trainer.train(10_000_000);
//    trainer
//        .agent
//        .save_to_file("data/q_table.csv")
//        .expect("Failed to save Q-table to file");

    let elapsed = start.elapsed();
    println!(
        "Training completed and Q-table saved to data/q_table.csv in {:?}",
        elapsed
    );
}
