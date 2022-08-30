use aws_sdk_neptune as neptune;
use axum::{routing::*, Extension, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

/// Handlers for different HTTP requests made to the server
mod handlers {
    /// Handlers that generate advisories when requested
    pub mod advisories;
    /// Handlers that handle adding and managing students and advisors
    pub mod people;
}
use handlers::*;

/// Shared state for accessing the database
#[allow(dead_code)]
#[derive(Debug)]
pub struct SharedState {
    config: aws_config::SdkConfig,
    client: neptune::Client,
}

/// Main async function run when executing the crate
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
        // Add routes to specific handler functions
        .route("/health", get(health)) // Health check
        .route("/info", get(info))
        .route("/people", post(people::add_person))
        .route("/", get(advisories::get_advisories))
        // Add shared state to all requests
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Healthcheck handler
async fn health() -> &'static str {
    "Healthy!"
}

/// Information on version and other fields set in the cargo manifest
#[derive(Debug, serde::Serialize)]
pub struct CrateInfo {
    name: &'static str,
    authors: Vec<&'static str>,
    version: &'static str,
    description: &'static str,
    license: &'static str,
    repository: &'static str,
}

/// Crate information handler
/// Uses [`CrateInfo`] struct
async fn info() -> Json<CrateInfo> {
    Json(CrateInfo {
        name: env!("CARGO_PKG_NAME"),
        authors: env!("CARGO_PKG_AUTHORS").split(',').collect(),
        version: env!("CARGO_PKG_VERSION"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        license: env!("CARGO_PKG_LICENSE"),
        repository: env!("CARGO_PKG_REPOSITORY"),
    })
}
