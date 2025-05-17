use dotenv::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;

pub async fn get_db_pool() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok(); // ← ここを一番最初に！

    // .envや設定から個別取得
    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "mydb".to_string());
    let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_pass = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port: u16 = env::var("DATABASE_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(5432);

    let options = PgConnectOptions::new()
        .host(&db_host)
        .port(db_port)
        .username(&db_user)
        .password(&db_pass)
        .database(&db_name);

    PgPoolOptions::new()
        .max_connections(100)
        .connect_with(options)
        .await
        .expect("failed to connect to DB")
}
