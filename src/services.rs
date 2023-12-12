use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;

#[get("/healthChecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health ckeck: API is up and running smoothly.";
    HttpResponse::Ok().json(json!({
      "staus": "success",
      "message": MESSAGE
    }))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker);
    conf.service(scope);
}
