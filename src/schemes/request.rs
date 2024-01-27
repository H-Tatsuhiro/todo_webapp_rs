use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateTodoReq {
    pub title: String,
    pub status: i32,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTodoReq {
    pub title: Option<String>,
    pub status: Option<i32>,
    pub description: Option<String>,
}