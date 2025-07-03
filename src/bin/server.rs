use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use rust_rl::{agents::q_agent::QAgent, environment::move_to_center::GridEnvironment, Environment};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Train the agent or load a saved Q-table.
    let agent = QAgent::load_from_file("data/q_table.json")
        .expect("Failed to load Agent with Q-table");
    println!("Agent loaded with Q-table.");

    // Wrap the agent in a Mutex and web::Data to share state safely.
    let agent_data = web::Data::new(agent);

    println!("Starting server on http://127.0.0.1:8000 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(agent_data.clone())
            .service(predict_all)
            .wrap(Cors::permissive())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

enum EnvironmentType {
    Grid,
}

fn predict_all_handler<E: Environment>(mut response: HttpResponseBuilder,
    agent: &QAgent,
) -> HttpResponse {
    response.json(agent.predict_all::<E>())
}

#[get("/predict_all")]
async fn predict_all(agent: web::Data<QAgent>) -> impl Responder {
    let env = EnvironmentType::Grid;
    match env {
        EnvironmentType::Grid => predict_all_handler::<GridEnvironment>(HttpResponse::Ok(), &agent),
    }
}