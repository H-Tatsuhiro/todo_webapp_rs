use sqlx::{MySqlPool, QueryBuilder};
use super::super::schemes as sch;
use sch::todo::Todo;
use sch::request::{CreateTodoReq, UpdateTodoReq};

pub async fn create_todo(pool: &MySqlPool, req: CreateTodoReq) -> Result<u64, sqlx::Error> {
   let result = sqlx::query!(
       "INSERT INTO todos (id, title, status, description) VALUES (?, ?, ?, ?)",
       req.id,
       req.title,
       req.status,
       req.description
   )
       .execute(pool)
       .await?;

    Ok(result.rows_affected())
}

pub async fn read_todos(pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, status, description, created_at, updated_at FROM todos"
    )
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

pub async fn read_todo(pool: &MySqlPool, id: i64) -> Result<Todo, sqlx::Error>{
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, status, description, created_at, updated_at FROM todos WHERE id = ?",
        id
    )
        .fetch_one(pool)
        .await?;

    Ok(todo)
}

pub async fn update_todo(pool: &MySqlPool, id: i64, req: UpdateTodoReq) -> Result<u64, sqlx::Error> {
    let mut builder = QueryBuilder::new("UPDATE todos SET");

    if let Some(req_title) = &req.title {
        builder.push(" title = ");
        builder.push_bind(req_title);
    }
    if let Some(req_status) = &req.status {
        builder.push(", status = ");
        builder.push_bind(req_status);
    }
    if let Some(req_description) = &req.description {
        builder.push(", description = ");
        builder.push_bind(req_description);
    }
    builder.push(", updated_at = NOW() WHERE id = ");
    builder.push_bind(id);

    let query = builder.build();
    let result = query.execute(pool).await?;

    Ok(result.rows_affected())
}

pub async fn delete_todo(pool: &MySqlPool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
