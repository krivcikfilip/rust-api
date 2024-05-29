use actix_web::web;
use sqlx::Error;

use crate::AppState;
use crate::body::create_user_body::CreateUserBody;
use crate::body::update_user_body::UpdateUserBody;
use crate::entity::user::User;

pub async fn get_users(app_state: web::Data<AppState<'_>>) -> Result<Vec<User>, Error> {
    app_state.context.users.get_users().await
}

pub async fn get_user(user_id: i32, app_state: web::Data<AppState<'_>>) -> Result<User, Error> {
    app_state.context.users.get_user(user_id).await
}

pub async fn create_user(
    body: &CreateUserBody,
    app_state: web::Data<AppState<'_>>,
) -> Result<(), Error> {
    app_state.context.users.create_user(&body).await
}

pub async fn update_user(
    user_id: i32,
    body: &UpdateUserBody,
    app_state: web::Data<AppState<'_>>,
) -> Result<(), Error> {
    app_state.context.users.update_user(user_id, &body).await
}
