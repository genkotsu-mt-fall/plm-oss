use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/healthz", get(health_check));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
