use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rust_rl::{agents::q_agent::QAgent, environment::move_to_center::GridEnvironment};

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

#[get("/predict_all")]
async fn predict_all(agent: web::Data<QAgent>) -> impl Responder {
    // Lock the agent and retrieve all predictions.
    let predictions = agent.predict_all::<GridEnvironment>();
    HttpResponse::Ok().json(predictions)
}