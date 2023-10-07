use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub mod controller;
pub mod model;
pub mod service;
// pub mod session;

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct Claims {
//     // username
//     pub sub: String,
//     pub exp: usize,
// }

#[derive(Debug)]
pub struct AuthBlacklist {
    pub id: Option<u64>,
    pub access_token_id: uuid::Uuid,
    pub access_token_exp: DateTime<Utc>,
    pub user_id: u64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Authorization {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub uuid: Option<uuid::Uuid>,
    // pub uuid: uuid::Uuid,
    pub client_type: Option<i16>,
    pub refresh_token: Option<uuid::Uuid>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub last_refresh_time: Option<DateTime<Utc>>,
    pub access_token_id: Option<uuid::Uuid>,
    pub access_token_exp: Option<DateTime<Utc>>,
    pub access_token_iat: Option<DateTime<Utc>>,
    pub is_enabled: Option<i16>,
}

#[derive(Debug)]
pub struct AuthorizationInfo {
    pub id: u64,
    pub scopes: Vec<String>,
}
