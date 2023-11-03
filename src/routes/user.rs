use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use aes_gcm_siv::aead::OsRng;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use log::error;
use serde::{Deserialize, Serialize};

use crate::{models::User, utils::generate_uuid, AppData};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user).service(register);
}

// todo: make these configurable
const DEFAULT_QUOTA: i64 = 1024 * 1024 * 1024;
const DEFAULT_USED: i64 = 0;
const DEFAULT_PERMISSIONS: i64 = 0;

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

#[derive(Serialize)]
struct RegisterResponse {
    id: i64,
    uuid: String,
    username: String,
    key: String,
    quota: i64,
    used: i64,
    permissions: i64,
}

#[post("/register")]
async fn register(info: web::Json<RegisterRequest>, data: Data<AppData>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(info.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let user = sqlx::query_as::<_, User>("SELECT id FROM users WHERE username = $1")
        .bind(&info.username)
        .fetch_optional(&data.pool)
        .await;

    if let Ok(Some(_)) = user {
        return HttpResponse::Conflict().body("Username already exists");
    }

    let uuid = generate_uuid();
    let key = generate_uuid();
    let user = match sqlx::query_as::<_, User>("INSERT INTO users (uuid, username, password, key, quota, used, permissions) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",)
        .bind(&uuid)
        .bind(&info.username)
        .bind(&password_hash)
        .bind(&key)
        .bind(DEFAULT_QUOTA)
        .bind(DEFAULT_USED)
        .bind(DEFAULT_PERMISSIONS)
        .fetch_one(&data.pool).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to create user: {}", e);
                return HttpResponse::InternalServerError().body("Failed to create user");
            }
        };

    HttpResponse::Ok().json(RegisterResponse {
        id: user.id,
        uuid: user.uuid,
        username: user.username,
        key: user.key,
        quota: user.quota,
        used: user.used,
        permissions: user.permissions,
    })
}
