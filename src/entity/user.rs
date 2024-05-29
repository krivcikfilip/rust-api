use serde::Serialize;
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl<'c> FromRow<'c, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(User {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            email: row.get(3),
        })
    }
}
