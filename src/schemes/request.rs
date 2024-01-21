use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateTodoReq {
    pub id: i64,
    pub title: String,
    pub status: i32,
    pub description: String,
}

#[derive(Debug)]
pub struct UpdateTodoReq {
    id: i64,
    title: String,
    status: i32,
}