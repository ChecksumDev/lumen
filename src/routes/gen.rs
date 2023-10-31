use crate::{
    models::{File, SXConfig, User},
    AppData,
};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

pub fn gen_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(config);
}

#[get("/")]
async fn index(data: Data<AppData>) -> Result<impl Responder, actix_web::Error> {
    let version = env!("CARGO_PKG_VERSION");

    let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&data.pool)
        .await.expect("Failed to get user count");

    let files_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
        .fetch_one(&data.pool)
        .await.expect("Failed to get file count");

    let total_size: i64 = sqlx::query_scalar("SELECT SUM(size) FROM files")
        .fetch_one(&data.pool)
        .await.expect("Failed to get total file size");

    let response = format!(
        "This server is running Lumen v{}\nServing {} users and {} files totaling {} MiB",
        version,
        users_count,
        files_count,
        total_size / 1024 / 1024
    );

    Ok(HttpResponse::Ok().body(response))
}

#[derive(Serialize, Deserialize)]
pub struct ConfigRequest {
    pub api_key: String,
}

#[get("/config")]
async fn config(req: web::Query<ConfigRequest>, data: Data<AppData>) -> impl Responder {
    let api_key = req.api_key.clone();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE key = $1")
        .bind(&api_key)
        .fetch_one(&data.pool)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    let config = SXConfig::new(&data.config.public_url, &api_key);

    HttpResponse::Ok()
        .append_header(("content-disposition", "attachment; filename=lumen.sxcu"))
        .json(config)
}
