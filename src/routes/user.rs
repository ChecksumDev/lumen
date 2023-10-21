use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use aes_gcm_siv::aead::OsRng;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::User, AppData};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user).service(register);
}

#[derive(Serialize)]
struct UserResponse {
    id: i64,
    uuid: String,
    username: String,
    quota: i64,
    used: i64,
    permissions: i64,
}

#[get("/users/{id}")]
async fn get_user(id: web::Path<i64>, data: Data<AppData>) -> impl Responder {
    let id = id.into_inner();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&data.pool)
        .await;

    if user.is_err() {
        return HttpResponse::NotFound().body(format!(
            "User {} not found, {}",
            &id,
            user.err().unwrap()
        ));
    }

    let user = user.unwrap();

    HttpResponse::Ok().json(UserResponse {
        id: user.id,
        uuid: user.uuid,
        username: user.username,
        quota: user.quota,
        used: user.used,
        permissions: user.permissions,
    })
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(info: web::Json<RegisterRequest>, data: Data<AppData>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(info.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&info.username)
        .fetch_one(&data.pool)
        .await;

    if user.is_ok() {
        return HttpResponse::BadRequest().body("Username already exists");
    }

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (uuid, username, password, key, quota, used, permissions) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&info.username)
    .bind(password_hash)
    .bind(Uuid::new_v4().to_string())
    .bind(1024 * 1024 * 1024)
    .bind(0)
    .bind(0)
    .fetch_one(&data.pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(user)
}
