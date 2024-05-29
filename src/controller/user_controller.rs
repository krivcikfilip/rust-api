use actix_web::{get, HttpResponse, post, put, Responder, web};

use crate::AppState;
use crate::body::create_user_body::CreateUserBody;
use crate::body::update_user_body::UpdateUserBody;
use crate::service::user_service;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(update_user);
}

#[get("/api/v1/users")]
async fn get_users(app_state: web::Data<AppState<'_>>) -> impl Responder {
    let users = user_service::get_users(app_state).await;

    match users {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(users) => HttpResponse::Ok().json(users),
    }
}

#[get("/api/v1/users/{id}")]
async fn get_user(user_id: web::Path<i32>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    let user_id = user_id.into_inner();

    let user = user_service::get_user(user_id, app_state).await;

    match user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(user) => HttpResponse::Ok().json(user),
    }
}

#[post("/api/v1/users")]
async fn create_user(
    body: web::Json<CreateUserBody>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let body = body.into_inner();

    let created_user = user_service::create_user(&body, app_state).await;

    match created_user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Created().finish(),
    }
}

#[put("/api/v1/users/{id}")]
async fn update_user(
    user_id: web::Path<i32>,
    body: web::Json<UpdateUserBody>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let body = body.into_inner();
    let user_id = user_id.into_inner();

    let updated_user = user_service::update_user(user_id, &body, app_state).await;

    match updated_user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
