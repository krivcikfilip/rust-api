use actix_web::{get, HttpResponse, post, Responder, web};
use uuid::Uuid;

use crate::AppState;
use crate::entity::user::User;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user);
    cfg.service(create_user);
}

#[get("/api/v1/users")]
async fn get_users(app_state: web::Data<AppState<'_>>) -> impl Responder {
    let users = app_state.context.users.get_users().await;

    match users {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(users) => HttpResponse::Ok().json(users),
    }
}

#[get("/api/v1/users/{id}")]
async fn get_user(
    user_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let user = app_state.context.users.get_user(&user_id).await;

    match user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(user) => HttpResponse::Ok().json(user),
    }
}

#[post("/api/v1/users")]
async fn create_user(user: web::Json<User>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    let mut user = user.into_inner();
    user.id = Uuid::new_v4().to_string();

    let created_user = app_state.context.users.create_user(&user).await;

    match created_user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Created().body(user.id),
    }
}
