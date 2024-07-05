use actix_web::{web, App, HttpServer};
use rust_api::controller::init_user_controller;
use rust_api::db_context::Database;
use rust_api::AppState;
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use rust_api::schedule::run_demo_schedule;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("empty DATABASE_URL env.");
    let db_context = Database::create(&url).await;
    println!("Connected to database {}", url);

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    run_demo_schedule(app_state.clone()).await;

    let host = "127.0.0.1";
    let port = 8080;
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(init_user_controller)
    })
    .bind((host, port))?;

    println!("Listening on: http://{}:{}", host, port);
    app.run().await
}
