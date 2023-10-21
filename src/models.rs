use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SXConfig {
    version: String,
    name: String,
    destination_type: String,
    request_method: String,
    request_url: String,
    headers: SXHeaders,
    body: String,
    url: String,
    deletion_url: String,
    error_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SXHeaders {
    x_api_key: String,
    x_file_name: String,
}

impl SXConfig {
    pub fn new(host: &String, api_key: &String) -> Self {
        Self {
            version: String::from("15.0.0"),
            name: String::from("Lumen"),
            destination_type: String::from("ImageUploader, TextUploader, FileUploader"),
            request_method: String::from("POST"),
            request_url: format!("{}/upload", host),
            headers: SXHeaders {
                x_api_key: api_key.clone(),
                x_file_name: String::from("{filename}"),
            },
            body: String::from("Binary"),
            url: format!("{}/{{json:id}}.{{json:ext}}?key={{json:key}}&nonce={{json:nonce}}", host),
            deletion_url: format!("{}/{{json:id}}.{{json:ext}}/delete?key={{json:key}}&nonce={{json:nonce}}&api_key={}", host, api_key),
            error_message: String::from("{response}"),
        }
    }
}

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
