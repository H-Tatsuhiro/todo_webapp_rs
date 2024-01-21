use actix_web::{get, post, web, App, HttpServer, Responder};
use todo_webapp_rs::schemes::request::CreateTodoReq;
use todo_webapp_rs::db_actions::create_conn::create_pool;
use todo_webapp_rs::db_actions::crud::{create_todo, read_todos, delete_todo};

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
async fn add_todo(posted_json: web::Json<CreateTodoReq>) -> impl Responder {
    let new_todo = CreateTodoReq {
        id: posted_json.id,
        title: posted_json.title.clone(),
        status: posted_json.status,
        description: posted_json.description.clone(),
    };
    let pool = create_pool().await.expect("error");
    let p = create_todo(&pool, new_todo).await.expect("error");

    format!("{:?}", p)
}

#[get("/delete_todo/{id}")]
async fn del_todo(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let pool = create_pool().await.expect("error");
    let deleted_rows = delete_todo(&pool, id).await.expect("error");

    format!("{deleted_rows} deleted.")
}

#[get("/get_todo")]
async fn get_todo() -> impl Responder {
    let pool = create_pool().await.expect("error");
    let todos = read_todos(&pool).await.expect("error");

    format!("{:?}", todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index).service(login).service(logout).service(register).service(add_todo).service(del_todo).service(get_todo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
