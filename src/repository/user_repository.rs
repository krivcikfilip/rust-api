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

    pub async fn get_user(&self, user_id: &String) -> Result<User, sqlx::Error> {
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

    pub async fn create_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, first_name, last_name, email)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&user.id)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .execute(&*self.pool)
        .await
        .map(|x| x.rows_affected())
    }
}
