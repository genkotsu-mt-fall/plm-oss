mod auth;
mod errors;
mod models;
mod responses;
mod routes;
mod services;

use auth::jwt::jwt_auth;
use auth::route::{login, signup};
use axum::http::HeaderValue;
use axum::routing::post;
use axum::{Router, http, middleware, routing::get};
use dotenvy::dotenv;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use routes::parts::{create_part, delete_part, get_part, get_parts, update_part};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
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
    let cors_origin = env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");

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

    // マイグレーションの自動実行
    if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
        error!("Failed to run database migrations: {}", err);
        panic!("Migration error");
    }

    let cors = CorsLayer::new()
        .allow_origin(
            cors_origin
                .parse::<HeaderValue>()
                .expect("Invalid CORS_ORIGIN value"),
        )
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let protected_routes = Router::new()
        .route("/parts", get(get_parts).post(create_part))
        .route(
            "/parts/{id}",
            get(get_part).put(update_part).delete(delete_part),
        )
        .route_layer(middleware::from_fn(jwt_auth));

    let app = Router::new()
        .route("/healthz", get(health_check))
        // .route("/parts", get(get_parts).post(create_part))
        // .route(
        //     "/parts/{id}",
        //     get(get_part).put(update_part).delete(delete_part),
        // )
        .route("/login", post(login))
        .route("/signup", post(signup))
        .merge(protected_routes)
        .with_state(pool)
        .layer(cors)
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
