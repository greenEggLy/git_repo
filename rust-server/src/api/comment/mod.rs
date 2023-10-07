use serde::{Deserialize, Serialize};

pub mod controller;
pub mod model;
pub mod service;

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Comment {
    content: String,
    score: i8,
    movie_id: i64,
    user_id: i64,
}
