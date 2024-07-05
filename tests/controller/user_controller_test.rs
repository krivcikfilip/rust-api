use actix_web::{App, test};
use sqlx::PgPool;
use rust_api::body::create_user_body::CreateUserBody;
use rust_api::controller::init_user_controller;
use rust_api::entity::user::User;
use crate::init_app_state;

#[sqlx::test]
async fn get_users_returns_ok(pool: PgPool) -> Result<(), sqlx::Error> {
    let app_state = init_app_state(pool).await;
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(init_user_controller)
    ).await;

    let create_user_body = CreateUserBody {
        email: "demo@demo.com".to_string(),
        first_name: "Demo".to_string(),
        last_name: "Demo".to_string()
    };

    app_state.context.user.create_user(&create_user_body).await?;

    let request = test::TestRequest::get().uri("/api/v1/users").to_request();
    let response: Vec<User> = test::call_and_read_body_json(&app, request).await;

    assert_eq!(response.len(), 1);
    assert_eq!(response.get(0).unwrap().first_name, create_user_body.first_name);
    Ok(())
}