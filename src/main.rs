use actix_web::{get, post, web, App, HttpServer, Responder};
use todo_webapp_rs::schemes::appstate::AppState;
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
async fn add(data: web::Data<AppState>, posted_json: web::Json<CreateTodoReq>) -> impl Responder {
    let new_todo = CreateTodoReq {
        title: posted_json.title.clone(),
        status: posted_json.status,
        description: posted_json.description.clone(),
    };
    let added_todos_num = create_todo(&data.db_pool, new_todo).await.expect("error");

    format!("{added_todos_num} added.")
}

#[get("/delete_todo/{id}")]
async fn delete(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let deleted_todos_num = delete_todo(&data.db_pool, id).await.expect("error");

    format!("{deleted_todos_num} deleted.")
}

#[get("/get_todos")]
async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let todos = read_todos(&data.db_pool).await.expect("error");

    format!("{:?}", todos)
}

#[get("/get_todo/{id}")]
async fn get_one(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let todo = read_todo(&data.db_pool, id).await.expect("error");

    format!("{todo:?}")
}

#[post("/update_todo/{id}")]
async fn update(data: web::Data<AppState>, path: web::Path<i64>, posted_json: web::Json<UpdateTodoReq>) -> impl Responder {
    let id = path.into_inner();
    let update_info = UpdateTodoReq {
        title: posted_json.title.clone(),
        status: posted_json.status,
        description: posted_json.description.clone()
    };
    let updated_todos_num = update_todo(&data.db_pool, id, update_info).await.expect("error");

    format!("{updated_todos_num} updated.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await.expect("Can't create database pool.");

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppState {
            db_pool: pool.clone()
        }))
        .service(index).service(login).service(logout).service(register).service(add).service(delete).service(get_all).service(get_one).service(update))
        .bind(("0.0.0.0", 50001))?
        .run()
        .await
}
