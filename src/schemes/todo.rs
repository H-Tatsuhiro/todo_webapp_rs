use sqlx::types::chrono;

#[derive(Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: i32,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
