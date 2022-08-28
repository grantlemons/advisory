use aws_config;
use aws_sdk_neptune as neptune;
use axum::{
    routing::*,
    Extension,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;

#[allow(dead_code)]
struct SharedState {
    config: aws_config::SdkConfig,
    client: neptune::Client,
}

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = neptune::Client::new(&config);
    let state = Arc::new(SharedState { config, client });

    let app = Router::new()
        // Add shared state to all requests
        .layer(Extension(state))
        // Add routes to specific handler functions
        .route("/", get(root)); // Health check

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Healthy!"
}
