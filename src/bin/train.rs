use std::{cell::RefCell, env::args, rc::Rc, time::Instant};

use rust_rl::{
    agents::q_agent::QAgent,
    environment::{move_to_center::GridEnvironment, tic_tac_toe::TicTacEnvironment},
    train, Agent, GRID_AGENT_SAVE_FILE_PATH, TIC_TAC_TOE_AGENT_SAVE_FILE_PATH,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const GRID_SIZE: (usize, usize) = (9, 9);
const EPISODES: u64 = 1_000_000;
fn main() {
    let a = args().nth(1).unwrap_or_else(|| "0".to_string());
    let start = Instant::now();
    let sty: ProgressStyle = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("#>-");
    let pb = ProgressBar::new(EPISODES);
    pb.set_style(sty.clone());
    match a.as_str() {
        "grid" => {
            pb.set_message("Training Grid Agent");
            train_grid_agent(10_000_000, pb);
        }
        "tic-tac-toe" => {
            pb.set_message("Training Tic Tac Toe Agent");
            train_tic_tac_toe_agent(10_000_000, pb);
        }
        _ => {
            println!("training all agents");
            let m: MultiProgress = MultiProgress::new();

            let pb = m.add(ProgressBar::new(EPISODES));
            pb.set_style(sty.clone());
            pb.set_message("Training Grid Agent");

            let pb2 = m.add(ProgressBar::new(EPISODES));
            pb2.set_style(sty);
            pb2.set_message("Training Tic Tac Toe Agent");

            let mut threads = vec![];
            threads.push(std::thread::spawn(|| train_grid_agent(EPISODES, pb)));
            threads.push(std::thread::spawn(|| {
                train_tic_tac_toe_agent(EPISODES, pb2)
            }));
            for thread in threads {
                thread.join().expect("Thread panicked");
            }
        }
    }

    let elapsed = start.elapsed();
    println!(
        "Training completed in {:.2?}. Q-table saved to data/q_table.",
        elapsed
    );
}

fn train_grid_agent(episodes: u64, pb: ProgressBar) {
    let mut env = GridEnvironment::new(GRID_SIZE.0, GRID_SIZE.1);
    let agent = Rc::new(RefCell::new(QAgent::new()));
    agent.borrow_mut().try_init(&env);
    let agents = [agent.clone() as Rc<RefCell<dyn Agent<GridEnvironment>>>];
    train::train(
        &mut env,
        &agents as &[Rc<RefCell<dyn Agent<GridEnvironment>>>],
        episodes,
        pb,
    );
    agent
        .borrow_mut()
        .save_to_file(GRID_AGENT_SAVE_FILE_PATH)
        .expect("Failed to save Q-table to file");
}

fn train_tic_tac_toe_agent(episodes: u64, pb: ProgressBar) {
    let mut env = TicTacEnvironment::new();
    let agent = Rc::new(RefCell::new(QAgent::new()));
    agent.borrow_mut().try_init(&env);
    let agents = [
        agent.clone() as Rc<RefCell<dyn Agent<TicTacEnvironment>>>,
        agent.clone() as Rc<RefCell<dyn Agent<TicTacEnvironment>>>,
    ];
    train::train(
        &mut env,
        &agents as &[Rc<RefCell<dyn Agent<TicTacEnvironment>>>],
        episodes,
        pb,
    );
    agent
        .borrow_mut()
        .save_to_file(TIC_TAC_TOE_AGENT_SAVE_FILE_PATH)
        .expect("Failed to save Q-table to file");
}
