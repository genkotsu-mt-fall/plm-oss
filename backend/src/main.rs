mod handlers;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use handlers::{create_part, delete_part, get_part, get_parts, update_part};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // tracingのロガー初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLが設定されていません。");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        // .expect("PostgreSQLの接続に失敗しました。");
        .unwrap_or_else(|err| {
            error!("Failed to connect to the database: {}", err);
            panic!("Database connection error");
        });

    info!("Successfully connected to the database");

    let app = Router::new()
        .route("/healthz", get(health_check))
        .route("/parts", get(get_parts).post(create_part))
        .route(
            "/parts/{id}",
            get(get_part).put(update_part).delete(delete_part),
        )
        .with_state(pool)
        .layer(TraceLayer::new_for_http()); // HTTPリクエストのログ出力

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap_or_else(|err| {
            error!("Failed to bind TCP listener: {}", err);
            panic!("Listener binding error");
        });
    info!("Server is running at http://localhost:3000");

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
    };
}
