use sqlx::{Pool, Postgres};

pub async fn get_pool() -> Pool<Postgres> {
    let pool: Pool<Postgres> = Pool::connect(&std::env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

    return  pool;
}

pub fn migrate() {
    sqlx::migrate!();
}