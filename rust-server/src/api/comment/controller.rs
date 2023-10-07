use crate::api::comment::{service, Comment};
use crate::lib::error;
use crate::state::AppState;
use actix_web::{post, web, HttpRequest, HttpResponse};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_comment);
}
#[post("/")]
pub async fn insert_comment(
    req_info: web::Json<Comment>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let res = service::insert_comment(&req_info, &state).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_error) => return Err(error::new(400007, "insert error", 422)),
    }
}
