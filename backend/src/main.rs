use axum::{routing::*, Extension, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use neo4rs::*;
use serde::Deserialize;
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
pub struct SharedState {
    graph: Arc<Graph>,
    num_advisories: i16,
    weights: Weights,
}

/// Weights from 0-10 used to assign importance to each possible parameter
#[derive(Debug, Deserialize)]
pub struct Weights {
    /// The importance that each student an an advisory has one of the advisors as a teacher
    /// Value from 0-10
    has_teacher: i8,
    /// The importance of biological sex diversity within advisories
    /// Value from 0-10
    sex_diverse: i8,
    /// The importance of grade diversity within advisories
    /// Value from 0-10
    grade_diverse: i8,
}

/// Main async function run when executing the crate
#[tokio::main]
async fn main() {
    // connect to datbase
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "test";
    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());

    // Create default settings for testing
    //TODO: Change from hardcoded weights and number of advisories to using an endpoint to set user config
    let weights = Weights {
        has_teacher: 10,
        sex_diverse: 5,
        grade_diverse: 5,
    };
    let state = Arc::new(SharedState {
        graph,
        num_advisories: 2,
        weights,
    });

    // Get SSL certificates from file
    // Refer to `README.md` for instruction on generating these
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

    // Axum setup and configuration
    let app = Router::new()
        // Add routes to specific handler functions
        .route("/health", get(health)) // Health check
        .route("/info", get(info))
        .route("/people/teacher", post(people::add_teacher))
        .route("/people/student", post(people::add_student))
        .route("/", get(advisories::get_advisories))
        // Add shared state to all requests
        .layer(Extension(state));

    // IP and Port to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // Bind axum app to configured IP and Port
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Healthcheck handler
/// Returns `Healthy!` if healthy
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

/// Crate information handler used to get information on the server
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
