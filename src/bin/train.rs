use std::{cell::RefCell, env::args, rc::Rc, time::Instant};

use rust_rl::{
    agents::{q_agent::QAgent, random_agent},
    environment::{move_to_center::GridEnvironment, tic_tac_toe::TicTacEnvironment},
    train, Agent,
};

const GRID_SIZE: (usize, usize) = (9, 9);

fn main() {
    let a = args().nth(1).unwrap_or_else(|| "grid".to_string());
    let start = Instant::now();
    let agent = Rc::new(RefCell::new(QAgent::new()));
    match a.as_str() {
        "grid" => {
            let mut env = GridEnvironment::new(GRID_SIZE.0, GRID_SIZE.1);
            agent.borrow_mut().try_init(&env);
            let agents = [agent.clone() as Rc<RefCell<dyn Agent<GridEnvironment>>>];
            train::train(
                &mut env,
                &agents as &[Rc<RefCell<dyn Agent<GridEnvironment>>>],
                1_000_000,
            );
        }
        "tic-tac-toe" => {
            let mut env = TicTacEnvironment::new();
            // let random_agent = Rc::new(RefCell::new(random_agent::RandomAgent::<TicTacEnvironment>::new()));
            agent.borrow_mut().try_init(&env);
            let agents = [
                agent.clone() as Rc<RefCell<dyn Agent<TicTacEnvironment>>>,
                agent.clone() as Rc<RefCell<dyn Agent<TicTacEnvironment>>>,
            ];
            train::train(
                &mut env,
                &agents as &[Rc<RefCell<dyn Agent<TicTacEnvironment>>>],
                10_000_000,
            );
            // let agents = [
            //     random_agent.clone(),
            //     agent.clone() as Rc<RefCell<dyn Agent<TicTacEnvironment>>>,
            // ];
            // train::train(
            //     &mut env,
            //     &agents as &[Rc<RefCell<dyn Agent<TicTacEnvironment>>>],
            //     10_000_000,
            // );
        }
        _ => {
            eprintln!("Unknown environment type: {}", a);
            return;
        }
    }

    agent
        .borrow_mut()
        .save_to_file("data/q_table.json")
        .expect("Failed to save Q-table to file");

    let elapsed = start.elapsed();
    println!(
        "Training completed and Q-table saved to data/q_table.json in {:?}",
        elapsed
    );
}
