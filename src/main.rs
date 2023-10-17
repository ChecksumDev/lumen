pub mod encryption;
pub mod models;
pub mod routes;
pub mod storage;

use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use dotenvy::dotenv;
use routes::{download, index, register, upload};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use storage::Storage;

struct AppData {
    pool: Pool<Sqlite>,
    storage: Storage,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let storage = Storage::new("data/uploads").await?;

    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("data/lumen.db")
                .create_if_missing(true),
        )
        .await?;

    // todo: support other databases (mysql, postgresql, etc)
    sqlx::migrate!().run(&pool).await?;
    let data = Data::new(AppData { pool, storage });
    let host = match std::env::var("HOST") {
        Ok(host) => host,
        Err(_) => {
            println!("The HOST environment variable is not set, defaulting to 127.0.0.1:8080");
            "127.0.0.1:8080".to_string()
        }
    };

    println!("Lumen is running on {}", host);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(register)
            .service(upload)
            .service(download)
    })
    .bind(host)?
    .run()
    .await?;

    Ok(())
}
