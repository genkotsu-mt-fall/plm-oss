mod handlers;

use axum::{Router, routing::get};
use handlers::{create_part, delete_part, get_part, get_parts, update_part};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let shared_part = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/healthz", get(health_check))
        .route("/parts", get(get_parts).post(create_part))
        .route(
            "/parts/{id}",
            get(get_part).put(update_part).delete(delete_part),
        )
        .with_state(shared_part);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
