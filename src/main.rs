use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};

use crate::db_context::Database;

mod controller;
mod db_context;
mod entity;
mod repository;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = String::from("postgres://user:password@127.0.0.1:55432/rust-db");
    let db_context = Database::new(&url).await;
    println!("Connected to database {}", url);

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_user_controller)
    })
    .bind(("127.0.0.1", 8080))?;

    println!("Listening on: {}", "http://127.0.0.1:8080");
    app.run().await
}
