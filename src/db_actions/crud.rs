use sqlx::MySqlPool;
use super::super::schemes as sch;
use sch::todo::Todo;
use sch::request::{CreateTodoReq, UpdateTodoReq};

pub async fn create_todo(pool: &MySqlPool, req: CreateTodoReq) -> Result<u64, sqlx::Error> {
   let result = sqlx::query!(
       "insert into todos (id, title, status, description) values (?, ?, ?, ?)",
       req.id,
       req.title,
       req.status,
       req.description
   )
       .execute(pool)
       .await?;

    Ok(result.last_insert_id())
}

pub async fn read_todos(pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        "select id, title, status, description, created_at, updated_at from todos"
    )
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

pub async fn read_todo() {

}

pub async fn update_todo(pool: &MySqlPool, req: UpdateTodoReq) {

}

pub async fn delete_todo(pool: &MySqlPool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("delete from todos where id = ?", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
