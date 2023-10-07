use crate::api::movie::service;
use crate::lib::error;
use crate::state::AppState;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(get_info_id);
    cfg.service(get_info_name);
}

#[derive(Deserialize)]
pub struct GetMovieInfoId {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct GetMovieInfoName {
    pub name: String,
}
#[get("/id")]
pub async fn get_info_id(
    req_info: web::Json<GetMovieInfoId>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let movie_data = service::get_by_id(req_info.id, &state).await?;
    Ok(HttpResponse::Ok().json(movie_data))
}

#[get("/name")]
pub async fn get_info_name(
    req_info: web::Json<GetMovieInfoName>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let movie_data = service::get_by_name(&req_info.name, &state).await?;
    Ok(HttpResponse::Ok().json(movie_data))
}
