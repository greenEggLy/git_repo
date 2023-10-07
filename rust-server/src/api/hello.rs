use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(add);
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[derive(Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct AddResponse {
    res: i32,
}

#[post("/add")]
async fn add(request: web::Json<AddRequest>) -> impl Responder {
    let response = AddResponse {
        res: request.a + request.b,
    };
    HttpResponse::Ok().json(response)
}
