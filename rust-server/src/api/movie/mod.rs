use chrono::{DateTime, Utc};
use serde::Serialize;

pub mod controller;
pub mod model;
pub mod service;

#[derive(Debug, sqlx::FromRow)]
pub struct Movie {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub synopsis: Option<String>,
    pub score: i8,
    pub score_num: u64,
}

impl Movie {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            release_date: None,
            synopsis: None,
            score: 0,
            score_num: 0,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct MovieInfo {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub synopsis: Option<String>,
    pub score: i8,
}
