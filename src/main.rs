#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

// Import modules
mod encryption;
mod models;
mod routes;
mod storage;
mod utils;

use actix_web::{
    web::{Data, PayloadConfig},
    App, HttpServer,
};
use anyhow::Result;
use dotenvy::dotenv;
use log::{info, warn};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use storage::Storage;

use crate::routes::{file::file_routes, gen::gen_routes, user::user_routes};

struct ConfigCache {
    public_url: String,
}

struct AppData {
    pool: Pool<Sqlite>,
    storage: Storage,
    config: ConfigCache,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    pretty_env_logger::init_custom_env("LUMEN_LOG");
    info!("Starting Lumen...");

    if std::env::var("LUMEN_LOG").unwrap_or_default() == "debug" {
        warn!("Lumen is running in debug mode. This is not recommended for production use.");
    }

    let storage = Storage::new("data").await?; // Create a new instance of the Storage struct
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("data/lumen.db")
                .create_if_missing(true),
        )
        .await?; // Create a new connection pool to the SQLite database

    let config = ConfigCache {
        public_url: std::env::var("PUBLIC_URL").expect("PUBLIC_URL not set in environment"),
    };

    info!("Running migrations...");

    // todo: support other databases (mysql, postgresql, etc)
    sqlx::migrate!().run(&pool).await?; // Run database migrations
    let data = Data::new(AppData { pool, storage, config });

    let bind = std::env::var("BIND").expect("BIND not set in environment");
    info!("Lumen is running on http://{}", bind);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(PayloadConfig::default().limit(1024 * 1024 * 100)) // 100MB
            .configure(gen_routes)
            .configure(user_routes)
            .configure(file_routes)
    })
    .bind(bind)?
    .run()
    .await?;

    Ok(())
}
