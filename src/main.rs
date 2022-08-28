use aws_config;
use aws_sdk_neptune as neptune;
use axum::{
    routing::*,
    Extension,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, sync::Arc, path::PathBuf};
use tokio;

#[allow(dead_code)]
struct SharedState {
    config: aws_config::SdkConfig,
    client: neptune::Client,
}

#[tokio::main]
async fn main() {
    // Add aws sdk conf and client as shared state
    let config = aws_config::load_from_env().await;
    let client = neptune::Client::new(&config);
    let state = Arc::new(SharedState { config, client });

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let app = Router::new()
        // Add shared state to all requests
        .layer(Extension(state))
        // Add routes to specific handler functions
        .route("/", get(root)); // Health check

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Healthy!"
}
