use actix_web::web;
use crate::AppState;

pub async  fn demo_schedule(app_state: web::Data<AppState<'_>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Started demo schedule at: {:?}", chrono::Utc::now());

    let users = app_state.context.user.get_users().await?;
    println!("Length of users: {}", users.len());

 
    Ok(())
}