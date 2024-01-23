use actix_web::{get, post, web, App, HttpServer, Responder};
use todo_webapp_rs::schemes::request::{CreateTodoReq, UpdateTodoReq};
use todo_webapp_rs::db_actions::create_conn::create_pool;
use todo_webapp_rs::db_actions::crud::{create_todo, read_todos, read_todo, update_todo, delete_todo};

#[get("/")]
async fn index() -> impl Responder {
    "top page"
}

#[get("/login")]
async fn login() -> impl Responder {
    "login"
}

#[get("/logout")]
async fn logout() -> impl Responder {
    "logout"
}

#[get("/register")]
async fn register() -> impl Responder {
    "register"
}

#[post("/add_todo")]
async fn add(posted_json: web::Json<CreateTodoReq>) -> impl Responder {
    let new_todo = CreateTodoReq {
        id: posted_json.id,
        title: posted_json.title.clone(),
        status: posted_json.status,
        description: posted_json.description.clone(),
    };
    let pool = create_pool().await.expect("error");
    let added_todos_num = create_todo(&pool, new_todo).await.expect("error");

    format!("{added_todos_num} added.")
}

#[get("/delete_todo/{id}")]
async fn delete(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let pool = create_pool().await.expect("error");
    let deleted_todos_num = delete_todo(&pool, id).await.expect("error");

    format!("{deleted_todos_num} deleted.")
}

#[get("/get_todos")]
async fn get_all() -> impl Responder {
    let pool = create_pool().await.expect("error");
    let todos = read_todos(&pool).await.expect("error");

    format!("{:?}", todos)
}

#[get("/get_todo/{id}")]
async fn get_one(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let pool = create_pool().await.expect("error");
    let todo = read_todo(&pool, id).await.expect("error");

    format!("{todo:?}")
}

#[post("/update_todo/{id}")]
async fn update(path: web::Path<i64>, posted_json: web::Json<UpdateTodoReq>) -> impl Responder {
    let id = path.into_inner();
    let update_info = UpdateTodoReq {
        title: posted_json.title.clone(),
        status: posted_json.status,
        description: posted_json.description.clone()
    };
    let pool = create_pool().await.expect("error");
    let updated_todos_num = update_todo(&pool, id, update_info).await.expect("error");

    format!("{updated_todos_num} updated.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index).service(login).service(logout).service(register).service(add).service(delete).service(get_all).service(get_one).service(update))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
