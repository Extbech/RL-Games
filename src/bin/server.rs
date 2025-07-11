use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, Query},
    App, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};
use rust_rl::{
    agents::q_agent::QAgent,
    environment::{
        move_to_center::{self, GridEnvironment},
        tic_tac_toe::{self, TicTacEnvironment},
    },
    Agent, Environment, GRID_AGENT_SAVE_FILE_PATH, TIC_TAC_TOE_AGENT_SAVE_FILE_PATH,
};
use serde::Deserialize;

struct AppState {
    grid_agent: QAgent,
    tic_tac_toe_agent: QAgent,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Train the agent or load a saved Q-table.
    let grid_agent = QAgent::load_from_file(GRID_AGENT_SAVE_FILE_PATH)
        .expect("Failed to load Agent with Q-table");
    let tic_tac_toe_agent = QAgent::load_from_file(TIC_TAC_TOE_AGENT_SAVE_FILE_PATH)
        .expect("Failed to load TicTacToe Agent with Q-table");
    let app_state = AppState {
        grid_agent,
        tic_tac_toe_agent,
    };
    println!("Agent loaded with Q-table.");

    // Wrap the agent in a Mutex and web::Data to share state safely.
    let app_state_data = web::Data::new(app_state);

    println!("Starting server on http://127.0.0.1:8000 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state_data.clone())
            .service((predict_all, predict))
            .wrap(Cors::permissive())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[derive(Deserialize)]
enum EnvironmentType {
    Grid,
    TicTacToe,
}

fn predict_all_handler<E: Environment>(
    mut response: HttpResponseBuilder,
    agent: &QAgent,
) -> HttpResponse {
    response.json(agent.predict_all::<E>())
}

#[get("/predict_all/{env}")]
async fn predict_all(
    agent: web::Data<AppState>,
    path: web::Path<EnvironmentType>,
) -> impl Responder {
    let env = path.into_inner();
    match env {
        EnvironmentType::Grid => {
            predict_all_handler::<GridEnvironment>(HttpResponse::Ok(), &agent.grid_agent)
        }
        EnvironmentType::TicTacToe => {
            predict_all_handler::<TicTacEnvironment>(HttpResponse::Ok(), &agent.tic_tac_toe_agent)
        }
    }
}

#[get("/predict/{env}")]
async fn predict(
    agent: web::Data<AppState>,
    path: web::Path<EnvironmentType>,
    query: Query<HashMap<String, String>>,
) -> impl Responder {
    let env = path.into_inner();
    let Some(state) = query.get("state") else {
        return HttpResponse::BadRequest().body("Missing 'state' query parameter");
    };
    match env {
        EnvironmentType::TicTacToe => {
            let obj = serde_json::from_str::<tic_tac_toe::Board>(&state).unwrap();
            let res = <QAgent as Agent<TicTacEnvironment>>::predict(&agent.tic_tac_toe_agent, &obj);
            HttpResponse::Ok().json(res)
        }
        EnvironmentType::Grid => {
            let obj = serde_json::from_str::<move_to_center::Board>(&state)
                .expect("Failed to deserialize GridEnvironment");
            let res = <QAgent as Agent<GridEnvironment>>::predict(&agent.grid_agent, &obj);
            HttpResponse::Ok().json(res)
        }
    }
}
