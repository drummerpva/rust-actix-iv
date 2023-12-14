use actix_web::{
    get, post,
    web::{scope, Data, Json, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    model::TaskModel,
    schema::{CreateTaskSchema, FilterOptions},
    AppState,
};

#[get("/healthChecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health ckeck: API is up and running smoothly.";
    HttpResponse::Ok().json(json!({
      "staus": "success",
      "message": MESSAGE
    }))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker)
        .service(create_task)
        .service(list_tasks);
    conf.service(scope);
}

#[post("/tasks")]
async fn create_task(body: Json<CreateTaskSchema>, context: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
    )
    .fetch_one(&context.db)
    .await
    {
        Ok(task) => {
            let note_response = json!({
                "status": "success",
                "task": task
            });
            return HttpResponse::Ok().json(note_response);
        }
        Err(error) => {
            println!("Error creating task: {:?}", error);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}",error)
            }));
        }
    }
}

#[get("/tasks")]
async fn list_tasks(
    filter_options: Query<FilterOptions>,
    context: Data<AppState>,
) -> impl Responder {
    let limit = filter_options.limit.unwrap_or(10);
    let offset = (filter_options.page.unwrap_or(1) - 1) * limit;
    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&context.db)
    .await
    {
        Ok(tasks) => {
            let tasks_response = json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });
            return HttpResponse::Ok().json(tasks_response);
        }
        Err(error) => {
            println!("Error listing tasks: {:?}", error);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}",error)
            }));
        }
    }
}
