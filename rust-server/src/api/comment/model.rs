use actix_web::{post, web, HttpResponse, Responder};

use crate::api::comment::Comment;
use crate::api::movie::model::update_movie_score;
use crate::lib::error;

pub async fn insert_comment(
    comment: &Comment,
    db: &sqlx::MySqlPool,
    log: &slog::Logger,
) -> Result<(), error::Error> {
    let res = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments(score, content, movie_id, user_id) VALUES(?, ?, ?, ?)",
    )
    .bind(comment.score)
    .bind(&comment.content)
    .bind(comment.movie_id)
    .bind(comment.user_id)
    .fetch_one(db)
    .await;
    if res.is_err() {
        error!(log, "{}", res.unwrap_err());
        return Err(error::new(12345, "insert error", 500));
    }
    let r = update_movie_score(comment.score, comment.movie_id, &db, &log).await;
    if r.is_err() {
        error!(log, "{}", res.unwrap_err());
        return Err(error::err500());
    }
    Ok(())
}
