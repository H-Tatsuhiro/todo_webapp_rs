use std::env;
use sqlx::MySqlPool;

pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("Empty db_url.");
    let pool = MySqlPool::connect(&db_url).await?;

    Ok(pool)
}

