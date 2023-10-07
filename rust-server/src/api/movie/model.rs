use crate::api::movie::{Movie, MovieInfo};
use crate::lib::error;
use slog::Logger;
use sqlx::{query, query_as, MySqlPool};

pub async fn get_movie_by_id(
    id: u64,
    db: &sqlx::MySqlPool,
    log: &slog::Logger,
) -> Result<Option<MovieInfo>, error::Error> {
    let r = sqlx::query_as::<_, MovieInfo>("SELECT * FROM movies WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await;

    match r {
        Ok(v) => Ok(v),
        Err(error) => {
            error!(log, "{}", error);
            Err(error::err500())
        }
    }
}

pub async fn get_movie_by_name(
    name: &str,
    db: &sqlx::MySqlPool,
    log: &Logger,
) -> Result<Option<MovieInfo>, error::Error> {
    let r = query_as::<_, MovieInfo>("SELECT * FROM movies WHERE name=?")
        .bind(name)
        .fetch_optional(db)
        .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn update_movie_score(
    new_score: i8,
    id: i64,
    db: &MySqlPool,
    log: &Logger,
) -> Result<(), error::Error> {
    let r = query(
        r#"
        UPDATE movies
        SET
        score = (score * score_num + ?) / (score_num + 1),
        score_num = score_num + 1 
        WHERE id = ?;"#,
    )
    .bind(new_score)
    .bind(id)
    .execute(db)
    .await;

    match r {
        Ok(_) => Ok(()),
        Err(err) => {
            error!(log, "{}", err);
            Err(error::err500())
        }
    }
}
