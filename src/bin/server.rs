use actix_cors::Cors;
use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};
use rust_rl::{
    agents::q_agent::QAgent,
    environment::{
        move_to_center::{self, GridEnvironment},
        tic_tac_toe::{self, TicTacEnvironment},
    },
    Agent, Environment,
};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Train the agent or load a saved Q-table.
    let agent =
        QAgent::load_from_file("data/q_table.json").expect("Failed to load Agent with Q-table");
    println!("Agent loaded with Q-table.");

    // Wrap the agent in a Mutex and web::Data to share state safely.
    let agent_data = web::Data::new(agent);

    println!("Starting server on http://127.0.0.1:8000 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(agent_data.clone())
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

#[get("/predict_all")]
async fn predict_all(agent: web::Data<QAgent>) -> impl Responder {
    let env = EnvironmentType::Grid;
    match env {
        EnvironmentType::Grid => predict_all_handler::<GridEnvironment>(HttpResponse::Ok(), &agent),
        _ => HttpResponse::BadRequest().body("Unsupported environment type"),
    }
}

#[get("/predict/{env}/{state}")]
async fn predict(
    agent: web::Data<QAgent>,
    path: web::Path<(EnvironmentType, String)>,
) -> impl Responder {
    let (env, state) = path.into_inner();
    match env {
        EnvironmentType::TicTacToe => {
            let obj = serde_json::from_str::<tic_tac_toe::Board>(state.as_str())
                .expect("Failed to deserialize TicTacEnvironment");
            let res = <QAgent as Agent<TicTacEnvironment>>::predict(&agent, &obj);
            HttpResponse::Ok().json(res)
        }
        EnvironmentType::Grid => {
            let obj = serde_json::from_str::<move_to_center::Board>(state.as_str())
                .expect("Failed to deserialize GridEnvironment");
            let res = <QAgent as Agent<GridEnvironment>>::predict(&agent, &obj);
            HttpResponse::Ok().json(res)
        }
    }
}
