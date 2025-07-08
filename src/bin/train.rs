use std::time::Instant;

use rust_rl::{
    agents::q_agent::QAgent,
    environment::move_to_center::{Board, GridEnvironment, MoveAction},
    train, Agent,
};

const GRID_SIZE: (usize, usize) = (9, 9);

fn main() {
    let start = Instant::now();
    let mut env = GridEnvironment::new(GRID_SIZE.0, GRID_SIZE.1);
    let mut agent = QAgent::new();
    agent.try_init(&env);
    train::train(&mut env, &mut agent, 1_000_000);
    agent
        .save_to_file("data/q_table.json")
        .expect("Failed to save Q-table to file");

    assert_eq!(
        <QAgent as Agent<GridEnvironment>>::predict(&agent, &Board { position: (2, 1) }),
        MoveAction::Up
    );

    let elapsed = start.elapsed();
    println!(
        "Training completed and Q-table saved to data/q_table.csv in {:?}",
        elapsed
    );
}
