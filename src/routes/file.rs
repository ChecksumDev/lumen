use actix_web::{
    get,
    http::header::HeaderValue,
    post,
    web::{self, Bytes, Data},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};
use uuid::Uuid;

use crate::{
    encryption::Cipher,
    models::{File, User},
    AppData,
};

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(upload)
        .service(download)
        .service(delete)
        .service(purge);
}

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
    let encrypted = match req.headers().get("x-encrypted") {
        Some(header) => header == HeaderValue::from_static("true"),
        None => true,
    };

    let file_size = bytes.len() as i64;
    if user.used + file_size > user.quota {
        return HttpResponse::PayloadTooLarge().body("Quota exceeded");
    }

    let file_name = req.headers().get("x-file-name");
    if file_name.is_none() {
        return HttpResponse::BadRequest().body("Missing file name");
    }

    let uuid = Uuid::new_v4().to_string();
    let file_name = file_name.unwrap().to_str().unwrap();
    let file_type = req.headers().get("content-type");

    if file_type.is_none() {
        return HttpResponse::BadRequest().body("Missing file type");
    }

    let file_type = file_type.unwrap().to_str().unwrap();

    let file_extension = file_type.split("/").last().unwrap();
    let file_hash = format!("{:x}", Sha3_512::digest(&bytes));

    let file_query = sqlx::query("INSERT INTO files (uuid, name, type, hash, size, user_id, encrypted) VALUES ($1, $2, $3, $4, $5, $6, $7)").bind(&uuid).bind(file_name).bind(file_type).bind(&file_hash).bind(file_size).bind(user.id);
    let (key, nonce) = if encrypted {
        let cipher = Cipher::default();
        let encrypted_bytes = cipher.encrypt(&bytes).expect("Failed to encrypt file");
        let encoded = cipher.to_base64();

        data.storage
            .save(String::from(&uuid), encrypted_bytes)
            .await.expect("Failed to save file");

        file_query.bind(true).execute(&data.pool).await.unwrap();
        (encoded.0, encoded.1)
    } else {
        data.storage
            .save(String::from(&uuid), bytes.to_vec())
            .await.expect("Failed to save file");

        file_query.bind(false).execute(&data.pool).await.unwrap();
        (String::new(), String::new())
    };

    sqlx::query("UPDATE users SET used = used + $1 WHERE id = $2")
        .bind(file_size)
        .bind(user.id)
        .execute(&data.pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(UploadResponse {
        id: String::from(&uuid),
        ext: String::from(file_extension),
        key,
        nonce,
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

    if file.encrypted && (info.key.is_empty() || info.nonce.is_empty()) {
        return HttpResponse::BadRequest().body("Missing decryption key or nonce");
    }

    let bytes = if file.encrypted {
        let cipher = Cipher::from_base64(&info.key, &info.nonce).expect("Failed to create cipher");
        let encrypted_bytes = data.storage.load(id).await.unwrap();
        cipher.decrypt(&encrypted_bytes).expect("Failed to decrypt file")
    } else {
        data.storage.load(id).await.unwrap()
    };

    HttpResponse::Ok()
        .append_header(("content-disposition", format!("filename=\"{}\"", file.name)))
        .append_header(("content-length", file.size.to_string()))
        .content_type(file.r#type)
        .body(bytes)
}

#[derive(Deserialize)]
struct DeleteRequest {
    api_key: String,
    key: String,
    nonce: String,
}

#[get("/{id}/delete")]
async fn delete(
    id: web::Path<String>,
    info: web::Query<DeleteRequest>,
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

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE key = $1")
        .bind(&info.api_key)
        .fetch_one(&data.pool)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let user = user.unwrap();

    if user.id != file.user_id {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    if file.encrypted && (info.key.is_empty() || info.nonce.is_empty()) {
        return HttpResponse::BadRequest().body("Missing decryption key or nonce");
    }

    let valid = if file.encrypted {
        let cipher = Cipher::from_base64(&info.key, &info.nonce).expect("Failed to create cipher");
        let encrypted_bytes = data.storage.load(id).await.unwrap();
        cipher.verify(&encrypted_bytes)
    } else {
        true
    };

    if !valid {
        return HttpResponse::Unauthorized().body("Invalid decryption key or nonce");
    }

    data.storage.delete(id).await.unwrap();

    sqlx::query("DELETE FROM files WHERE uuid = $1")
        .bind(&id)
        .execute(&data.pool)
        .await
        .unwrap();

    sqlx::query("UPDATE users SET used = used - $1 WHERE id = $2")
        .bind(file.size)
        .bind(user.id)
        .execute(&data.pool)
        .await
        .unwrap();

    HttpResponse::Ok().body("Deleted file")
}

#[post("/purge")]
async fn purge(info: HttpRequest, data: Data<AppData>) -> impl Responder {
    let api_key = info.headers().get("x-api-key");

    if api_key.is_none() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let api_key = api_key.unwrap().to_str().unwrap();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE key = $1")
        .bind(&api_key)
        .fetch_one(&data.pool)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let user = user.unwrap();

    let files = sqlx::query_as::<_, File>("SELECT * FROM files WHERE user_id = $1")
        .bind(user.id)
        .fetch_all(&data.pool)
        .await
        .unwrap();

    for file in files {
        data.storage.delete(file.uuid).await.unwrap();
    }

    sqlx::query("DELETE FROM files WHERE user_id = $1")
        .bind(user.id)
        .execute(&data.pool)
        .await
        .unwrap();

    sqlx::query("UPDATE users SET used = 0 WHERE id = $1")
        .bind(user.id)
        .execute(&data.pool)
        .await
        .unwrap();

    HttpResponse::Ok().body("Purged all files")
}
