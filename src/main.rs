use std::env;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;

use crate::db_context::Database;

mod body;
mod controller;
mod db_context;
mod entity;
mod repository;
mod service;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("empty DATABASE_URL env.");
    let db_context = Database::new(&url).await;
    println!("Connected to database {}", url);

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let host = "127.0.0.1";
    let port = 8080;
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_user_controller)
    })
    .bind((host, port))?;

    println!("Listening on: {}:{}", "host", port);
    app.run().await
}
