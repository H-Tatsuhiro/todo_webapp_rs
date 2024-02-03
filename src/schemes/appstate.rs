use sqlx::MySqlPool;

pub struct AppState {
    pub db_pool: MySqlPool
}