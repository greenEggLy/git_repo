use crate::api::comment::{model, Comment};
use crate::state::AppState;
use actix_web::web;
use sqlx::error;

pub async fn insert_comment(
    comment: &Comment,
    state: &web::Data<AppState>,
) -> Result<(), error::Error> {
    let result = model::insert_comment(comment, &state.db, &state.log).await;
    if result.is_err() {
        result.unwrap_err();
    }
    Ok(())
}
