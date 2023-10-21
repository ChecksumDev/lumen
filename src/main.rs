pub mod encryption;
pub mod models;
pub mod routes;
pub mod storage;

use actix_web::{
    web::{Data, PayloadConfig},
    App, HttpServer,
};
use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use storage::Storage;

use crate::routes::{file::file_routes, user::user_routes, gen::gen_routes};

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

    let storage = Storage::new("data").await?;
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("data/lumen.db")
                .create_if_missing(true),
        )
        .await?;

    let config = ConfigCache {
        public_url: std::env::var("PUBLIC_URL").expect("PUBLIC_URL not set in environment"),
    };

    // todo: support other databases (mysql, postgresql, etc)
    sqlx::migrate!().run(&pool).await?;
    let data = Data::new(AppData { pool, config, storage });

    let bind = std::env::var("BIND").expect("BIND not set in environment");
    println!("Lumen is running on {}", bind);
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
