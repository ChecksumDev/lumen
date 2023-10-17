use actix_web::{
    get, post,
    web::{self, Bytes, Data},
    HttpRequest, HttpResponse, Responder,
};
use aes_gcm_siv::aead::OsRng;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Sha3_512;
use uuid::Uuid;

use crate::{
    encryption::Encryption,
    models::{File, User},
    storage::Storage,
    AppData,
};

#[derive(Serialize)]
struct UploadResponse {
    id: String,
    ext: String,
    key: String,
    nonce: String,
}

#[post("/upload")]
async fn upload(bytes: Bytes, req: HttpRequest, data: Data<AppData>) -> impl Responder {
    let api_key = req.headers().get("x-api-key");

    if api_key.is_none() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let api_key = api_key.unwrap().to_str().unwrap();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE key = $1")
        .bind(api_key)
        .fetch_one(&data.pool)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let user = user.unwrap();

    let file_size = bytes.len() as i64;

    if user.used + file_size > user.quota {
        return HttpResponse::BadRequest().body("Quota exceeded");
    }

    let file_name = req.headers().get("x-file-name");

    if file_name.is_none() {
        return HttpResponse::BadRequest().body("Invalid file name");
    }

    let uuid = Uuid::new_v4().to_string();
    let file_name = file_name.unwrap().to_str().unwrap();
    let file_type = req.headers().get("content-type").unwrap().to_str().unwrap();
    let file_extension = file_type.split("/").last().unwrap();
    let file_hash = format!("{:x}", Sha3_512::digest(&bytes));

    let encryption = Encryption::default();
    let encrypted_bytes = encryption.encrypt(&bytes);

    let encoded_key = general_purpose::URL_SAFE_NO_PAD.encode(encryption.key);
    let encoded_nonce = general_purpose::URL_SAFE_NO_PAD.encode(encryption.nonce);

    let storage = Storage::new(String::from("data")).await;
    storage
        .save(String::from(&uuid), &encrypted_bytes)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO files (uuid, name, type, hash, size, user_id) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&uuid)
    .bind(file_name)
    .bind(file_type)
    .bind(&file_hash)
    .bind(file_size)
    .bind(user.id)
    .execute(&data.pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(UploadResponse {
        id: String::from(&uuid),
        ext: String::from(file_extension),
        key: encoded_key,
        nonce: encoded_nonce,
    })
}

#[derive(Deserialize)]
struct DownloadRequest {
    key: String,
    nonce: String,
}

#[get("/{id}")]
async fn download(
    id: web::Path<String>,
    info: web::Query<DownloadRequest>,
    data: Data<AppData>,
) -> impl Responder {
    let id = id.into_inner();
    let id = id.split(".").next().unwrap();

    let file = sqlx::query_as::<_, File>("SELECT * FROM files WHERE uuid = $1")
        .bind(&id)
        .fetch_one(&data.pool)
        .await;

    if file.is_err() {
        return HttpResponse::NotFound().body(format!(
            "File {} not found, {}",
            &id,
            file.err().unwrap()
        ));
    }

    let file = file.unwrap();

    let encryption = Encryption {
        key: general_purpose::URL_SAFE_NO_PAD
            .decode(info.key.as_bytes())
            .unwrap(),
        nonce: general_purpose::URL_SAFE_NO_PAD
            .decode(info.nonce.as_bytes())
            .unwrap(),
    };

    let storage = Storage::new(String::from("data")).await;
    let encrypted_bytes = storage.load(id).await.unwrap();

    let bytes = encryption.decrypt(&encrypted_bytes);

    HttpResponse::Ok()
        .append_header(("content-disposition", format!("filename=\"{}\"", file.name)))
        .append_header(("content-length", file.size.to_string()))
        .content_type(file.r#type)
        .body(bytes)
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

#[get("/")]
async fn index(data: Data<AppData>) -> impl Responder {
    let version = env!("CARGO_PKG_VERSION");
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&data.pool)
        .await
        .unwrap();

    let files = sqlx::query_as::<_, File>("SELECT * FROM files")
        .fetch_all(&data.pool)
        .await
        .unwrap();

    HttpResponse::Ok().body(format!(
        "This server is running Lumen v{}\nServing {} users and {} files",
        version,
        users.len(),
        files.len()
    ))
}
