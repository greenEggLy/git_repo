use actix_web::web;
use std::os::macos::raw::stat;

use crate::api::movie::model::update_movie_score;
use crate::api::movie::{model, Movie, MovieInfo};
use crate::lib::error;
use crate::state::AppState;

pub async fn get_by_id(
    id: u64,
    state: &web::Data<AppState>,
) -> Result<Option<MovieInfo>, error::Error> {
    let result = model::get_movie_by_id(id, &state.db, &state.log).await?;
    Ok(result)
}

pub async fn get_by_name(
    name: &str,
    state: &web::Data<AppState>,
) -> Result<Option<MovieInfo>, error::Error> {
    let result = model::get_movie_by_name(name, &state.db, &state.log).await?;
    Ok(result)
}

pub async fn update_score(
    id: i64,
    score: i8,
    state: &web::Data<AppState>,
) -> Result<(), error::Error> {
    let result = update_movie_score(score, id, &state.db, &state.log).await?;
    Ok(result)
}
