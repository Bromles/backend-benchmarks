use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::response::IntoResponse;

async fn handler() -> impl IntoResponse {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}