use axum::{routing::get, Router};
use axum::response::IntoResponse;

async fn handler() -> impl IntoResponse {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(addr, app)
        .await
        .unwrap();
}