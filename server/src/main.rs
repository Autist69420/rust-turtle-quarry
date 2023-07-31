use axum::http::Method;
use axum::{response::Html, routing::get, Router};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use server::websocket;

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/ws/", get(websocket::websocket_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// for now
async fn index() -> Html<&'static str> {
    Html("I have to code this.")
}
