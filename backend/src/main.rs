//! A crate for running the backend of an application used to sort students into advisories based on
//! specific criteria with weighted values that can be configured via an endpoint.
//!
//! ---
//!
//! **Notes**
//!
//! A custom fork of neo4rs is used to add functionality for handling vectors as a return type from neo4j

use axum::{routing::*, Extension, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

/// Various functions and structs used elsewhere in the code
mod lib {
    pub(crate) mod advisory {
        pub(crate) mod advisory;
        pub(crate) mod weights;
    }
    pub(crate) mod forms {
        pub(crate) mod advisory;
        pub(crate) mod student;
        pub(crate) mod students;
        pub(crate) mod teacher;
        pub(crate) mod teachers;
    }
    pub(crate) mod people {
        pub(crate) mod grade;
        pub(crate) mod sex;
        pub(crate) mod student;
        pub(crate) mod teacher;
    }
}
#[allow(unused_imports)]
use lib::advisory;
use lib::forms;
use lib::people;

/// Handlers for different HTTP requests made to the server
mod handlers {
    /// Handlers that generate advisories when requested
    pub(crate) mod advisories;
    /// Handlers that handle adding and managing students and advisors
    pub(crate) mod people;
}
#[cfg(test)]
mod tests {
    mod adding_people;
    mod advisory_building;
    mod info_handlers;
}

/// Verify trait for input validation
pub trait Verify {
    fn verify(&self) -> bool;
}

/// Shared state for accessing the database
#[allow(dead_code)]
pub(crate) struct SharedState {
    graph: Arc<neo4rs::Graph>,
}

/// Ports bound to for http and https connections
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

/// Main async function run when executing the crate
#[tokio::main]
async fn main() {
    // Setup logger
    setup_logger().expect("Unable to setup logger with fern");

    // Connect to datbase
    let uri = match std::env::var("DOCKER") {
        Ok(_) => "database:7687",
        Err(_) => "localhost:7687",
    };
    let user = "neo4j";
    let pass = "test";
    let graph = Arc::new(neo4rs::Graph::new(uri, user, pass).await.unwrap());
    let state = Arc::new(SharedState { graph });

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

    // Ports for http & https redirect
    let ports = Ports {
        http: 7878,
        https: 3000,
    };

    // IP and Port to bind to
    let addr = match std::env::var("DOCKER") {
        Ok(_) => SocketAddr::from(([0, 0, 0, 0], ports.https)),
        Err(_) => SocketAddr::from(([127, 0, 0, 1], ports.https)),
    };
    log::info!("listening on {}", addr);

    // spawn a second server to redirect http requests to the https server
    tokio::spawn(redirect_http_to_https(ports, addr.ip()));

    // Bind axum app to configured IP and Port
    axum_server::bind_rustls(addr, config)
        .serve(app(state).into_make_service())
        .await
        .unwrap();
}

fn app(state: Arc<SharedState>) -> Router {
    // Axum setup and configuration
    Router::new()
        // Add routes to specific handler functions
        .route("/health", get(health)) // Health check
        .route("/info", get(info))
        .route(
            "/people/teacher",
            post(handlers::people::add_teacher_handler),
        )
        .route(
            "/people/student",
            post(handlers::people::add_student_handler),
        )
        .route(
            "/people/teacher/bulk",
            post(handlers::people::add_teacher_bulk),
        )
        .route(
            "/people/student/bulk",
            post(handlers::people::add_student_bulk),
        )
        .route("/", put(handlers::advisories::get_advisories))
        // Add shared state to all requests
        .layer(Extension(state))
}

/// Function to redirect http requests to https
async fn redirect_http_to_https(ports: Ports, ip: IpAddr) {
    use axum::{
        extract::Host,
        handler::Handler,
        http::{uri, StatusCode, Uri},
        response::Redirect,
        BoxError,
    };

    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                log::warn!("failed to convert URI to HTTPS: {}", error);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::new(ip, ports.http);
    log::info!("http redirect listening on {}", addr);

    axum_server::bind(addr)
        .serve(Handler::into_make_service(redirect))
        .await
        .unwrap();
}

/// Healthcheck handler
///
/// Returns `Healthy!` if healthy
async fn health() -> &'static str {
    "Healthy!"
}

/// Information on version and other fields set in the cargo manifest
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
pub(crate) struct CrateInfo {
    name: &'static str,
    authors: Vec<&'static str>,
    version: &'static str,
    description: &'static str,
    license: &'static str,
    repository: &'static str,
}

/// Crate information handler used to get information on the server
///
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

/// Logger configuration using [`fern`]
fn setup_logger() -> Result<(), fern::InitError> {
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
        .chain(fern::log_file(format!(
            "/logs/{}.log",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M]")
        ))?)
        .apply()?;
    Ok(())
}
