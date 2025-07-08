use std::{cell::RefCell, rc::Rc, time::Instant};

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
    let agent = Rc::new(RefCell::new(agent));
    let agents = [agent.clone() as Rc<RefCell<dyn Agent<GridEnvironment>>>];
    train::train(&mut env, &agents as &[Rc<RefCell<dyn Agent<GridEnvironment>>>], 1_000_000);
    agent.borrow_mut()
        .save_to_file("data/q_table.json")
        .expect("Failed to save Q-table to file");

    let elapsed = start.elapsed();
    println!(
        "Training completed and Q-table saved to data/q_table.csv in {:?}",
        elapsed
    );
}
