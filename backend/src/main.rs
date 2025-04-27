mod handlers;

use axum::Router;
use axum::routing::get;
use handlers::create_part;
use handlers::get_parts;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let shared_part = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/healthz", get(health_check))
        .route("/parts", get(get_parts).post(create_part))
        .with_state(shared_part);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
