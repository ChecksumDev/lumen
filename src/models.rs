use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub key: String,
    pub quota: i64,
    pub used: i64,
    pub permissions: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub r#type: String,
    pub hash: String,
    pub size: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}
