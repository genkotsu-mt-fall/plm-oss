mod handlers;

use axum::{Router, routing::get, routing::post};
use dotenvy::dotenv;
// use handlers::{create_part, delete_part, get_part, get_parts, update_part};
use handlers::create_part;
use sqlx::postgres::PgPoolOptions;
// use std::{env, sync::Arc};
use std::env;
// use tokio::{net::TcpListener, sync::Mutex};
use tokio::net::TcpListener;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // .envファイルから環境変数読み込み
    dotenv().ok();

    // DATABASE_URLを取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLが設定されていません。");

    // PostgreSQLへの接続プール作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("PostgreSQLの接続に失敗しました。");

    println!("データベース接続成功");

    // let shared_part = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/healthz", get(health_check))
        // .route("/parts", get(get_parts).post(create_part))
        // .route(
        //     "/parts/{id}",
        //     get(get_part).put(update_part).delete(delete_part),
        // )
        .route("/parts", post(create_part))
        .with_state(pool);
    // .with_state(Arc::new(pool));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
