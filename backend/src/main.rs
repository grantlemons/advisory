#![warn(missing_docs, clippy::missing_docs_in_private_items)]
//! A crate for running the backend of an application used to sort students into advisories based on
//! specific criteria with weighted values that can be configured via an endpoint.
//!
//! ---
//!
//! **Notes**
//!
//! A custom fork of neo4rs is used to add functionality for handling vectors as a return type from neo4j
use anyhow::{Context, Result};
use axum::{routing::*, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

/// Handlers for different HTTP requests made to the server
mod handlers {
    /// Handlers that generate advisories when requested
    mod advisories;
    /// Handlers for server info and health check
    mod info;
    /// Handlers that handle adding and managing students and advisors
    mod people;

    pub(crate) use advisories::*;
    pub(crate) use info::*;
    pub(crate) use people::*;
}

/// Functions for verifying the JWT of HTTP(S) requests
mod auth;

/// Shared state for accessing the database
#[allow(dead_code)]
#[derive(Clone)]
struct SharedState {
    /// Graph for database access
    graph: Option<Arc<neo4rs::Graph>>,
    /// Keyset for JWT decoding (auth)
    keyset: Arc<jsonwebtokens_cognito::KeySet>,
    /// Verifier for JWT decoding (auth)
    /// Stored in state so it doesn't need to be generated each time
    verifier: Arc<jsonwebtokens::Verifier>,
}

/// Main async function run when executing the crate
#[tokio::main]
async fn main() -> Result<()> {
    // Setup logger
    setup_logger()?;

    // Connect to database
    let user = "neo4j";
    let uri = match std::env::var("DB_ADDRESS") {
        Ok(val) => format!("{}:7687", val),
        Err(_) => "localhost:7687".to_owned(),
    };
    let pass = match std::env::var("DB_PASS") {
        Ok(val) => val,
        Err(_) => "test".to_owned(),
    };
    let graph = Arc::new(
        neo4rs::Graph::new(&uri, user, &pass)
            .await
            .context("Unable to connect to database")?,
    );

    // JSON webtoken setup
    let keyset =
        Arc::new(jsonwebtokens_cognito::KeySet::new("us-east-1", "us-east-1_Ye96rGbqV").unwrap());
    keyset.prefetch_jwks().await.unwrap();
    let verifier = Arc::new(
        keyset
            .new_access_token_verifier(&["5c6eva8nctpb3aug8l0teak36v"])
            .build()?,
    );

    // State to be accessed by handlers
    let state = SharedState {
        graph: Some(graph),
        keyset,
        verifier,
    };

    // IP and Port to bind to
    let addr = SocketAddr::from(([0, 0, 0, 0], 81));
    log::info!("listening on {}", addr);

    // Use HTTP or HTTPS depending on ENV environment variable
    let use_tls = match std::env::var("ENV") {
        Ok(val) => match val.as_str() {
            "DOCKER" => true,
            "ECS" => false,
            _ => false,
        },
        Err(_) => true,
    };

    // Bind axum app to configured IP and Port
    // Use TLS (HTTPS) depending on the previous check
    match use_tls {
        false => {
            axum::Server::bind(&addr)
                .serve(app(state).into_make_service())
                .await?;
        }
        true => {
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
            .await?;

            axum_server::bind_rustls(addr, config)
                .serve(app(state).into_make_service())
                .await?;
        }
    }
    Ok(())
}

/// Configure routes for axum server
fn app(state: SharedState) -> Router {
    // Axum setup and configuration
    let api_router = Router::new()
        // Add routes to specific handler functions
        .route("/health", get(handlers::get_health)) // Health check
        .route("/info", get(handlers::get_info))
        .route(
            "/people",
            delete(handlers::clear_people_handler).get(handlers::get_people_handler),
        )
        .route("/people/ban", post(handlers::ban_pair_handler))
        .route("/people/teacher", post(handlers::add_teacher_handler))
        .route("/people/teacher", get(handlers::get_teachers_handler))
        .route("/people/student", post(handlers::add_student_handler))
        .route("/people/student", get(handlers::get_students_handler))
        .route("/people/teacher/bulk", post(handlers::add_teacher_bulk))
        .route("/people/student/bulk", post(handlers::add_student_bulk))
        .route("/", put(handlers::get_advisories));
    Router::new()
        // add /api before all routes
        .nest("/api", api_router)
        // jsonwebtoken auth layer
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth::auth,
        ))
        // Add shared state to all requests
        .with_state(state)
}

/// Logger configuration using [`fern`]
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::log_file(format!(
                "/logs/{}.log",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M]")
            ))
            .context("Unable to open log file")?,
        )
        .apply()
        .context("Failed to dispatch logger")?;
    Ok(())
}
