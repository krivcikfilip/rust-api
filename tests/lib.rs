mod controller;

use std::sync::{Arc, Mutex};
use actix_web::web::Data;
use sqlx::PgPool;
use rust_api::AppState;
use rust_api::db_context::Database;

async fn init_app_state(pool: PgPool) -> Data<AppState<'static>> {
    let db_context = Database::create_with_pool(pool);

    Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    })
}