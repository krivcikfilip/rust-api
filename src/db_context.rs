use std::marker::PhantomData;
use std::sync::Arc;

use sqlx::{FromRow, PgPool, Pool, Postgres};
use sqlx::postgres::PgRow;

use crate::entity::user::User;

pub struct Table<'c, T>
where
    T: FromRow<'c, PgRow>,
{
    pub pool: Arc<PgPool>,
    _from_row: fn(&'c PgRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, PgRow>,
{
    fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub pool: Arc<Pool<Postgres>>,
    pub user: Arc<Table<'c, User>>,
}

impl<'a> Database<'a> {
    pub fn create_with_pool(pool: PgPool) -> Database<'a> {
        let arc_pool = Arc::new(pool);

        Database {
            pool: arc_pool.clone(),
            user: Arc::from(Table::new(arc_pool.clone())),
        }
    }

    pub async fn create(url: &String) -> Database<'a> {
        let pool = PgPool::connect(&url).await.unwrap();
        Self::create_with_pool(pool)
    }
}
