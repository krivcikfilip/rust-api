use crate::body::create_user_body::CreateUserBody;
use crate::body::update_user_body::UpdateUserBody;
use crate::db_context::Table;
use crate::entity::user::User;

impl<'c> Table<'c, User> {
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, first_name, last_name, email
            FROM users
            "#,
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn get_user(&self, user_id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, first_name, last_name, email
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn create_user(&self, body: &CreateUserBody) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (first_name, last_name, email)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&body.first_name)
        .bind(&body.last_name)
        .bind(&body.email)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user(
        &self,
        user_id: i32,
        body: &UpdateUserBody,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users SET first_name = $1, last_name = $2, email = $3
            WHERE id = $4
            "#,
        )
        .bind(&body.first_name)
        .bind(&body.last_name)
        .bind(&body.email)
        .bind(&user_id)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
