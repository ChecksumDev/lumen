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

    // todo: support other databases (mysql, postgresql, etc)
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("lumen.db")
                .create_if_missing(true),
        )
        .await
        .unwrap();

    let storage = Storage::new(String::from("data")).await;

    sqlx::migrate!().run(&pool).await.unwrap();
    let data = Data::new(AppData { pool, storage });

    println!("Lumen is running on {}", std::env::var("HOST").unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(register)
            .service(upload)
            .service(download)
    })
    .bind(std::env::var("HOST")?)?
    .run()
    .await?;

    Ok(())
}
