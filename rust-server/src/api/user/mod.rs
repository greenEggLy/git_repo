use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod controller;
pub mod model;
pub mod service;

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub uuid: Option<uuid::Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<uuid::Uuid>,
    pub mobile: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub is_del: Option<i16>,
    pub is_enabled: Option<i16>,
    pub last_login_time: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub user_type: Option<i16>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: None,
            uuid: None,
            username: None,
            password: None,
            salt: None,
            mobile: None,
            create_time: None,
            update_time: None,
            is_del: None,
            is_enabled: None,
            last_login_time: None,
            last_login_ip: None,
            user_type: None,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub mobile: Option<String>,
    pub last_login_time: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub user_type: i16,
}
