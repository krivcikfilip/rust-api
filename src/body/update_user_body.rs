use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
