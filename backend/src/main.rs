//! A crate for running the backend of an application used to sort students into advisories based on
//! specific criteria with weighted values that can be configured via an endpoint.
//!
//! ---
//!
//! **Notes**
//!
//! A custom fork of neo4rs is used to add functionality for handling vectors as a return type from neo4j

use axum::{handler::HandlerWithoutStateExt, routing::*, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

/// Various functions and structs used elsewhere in the code
mod lib {
    pub(crate) mod advisories {
        mod advisory;
        pub(crate) mod builder;

        pub(crate) use advisory::Advisory;
    }
    mod settings;
    mod weights;
    pub(crate) mod people {
        mod grade;
        mod person;
        mod sex;
        mod student;
        mod teacher;

        pub(crate) use grade::Grade;
        pub(crate) use person::Person;
        pub(crate) use sex::Sex;
        pub(crate) use student::Student;
        pub(crate) use teacher::Teacher;
    }
    pub(crate) mod database;

    pub(crate) use settings::Settings;
    pub(crate) use weights::Weights;
}
use lib::advisories;
use lib::database;
use lib::people;
use lib::Settings;
use lib::Weights;

/// Handlers for different HTTP requests made to the server
mod handlers {
    /// Handlers that generate advisories when requested
    pub(crate) mod advisories;
    /// Handlers for server info and health check
    pub(crate) mod info;
    /// Handlers that handle adding and managing students and advisors
    pub(crate) mod people;
}
use handlers::advisories::*;
use handlers::info::*;
use handlers::people::*;
mod auth;

#[cfg(test)]
mod tests {
    mod info_handlers;
}

/// Verify trait for input validation
trait Verify {
    fn verify(&self) -> bool;
}

/// Shared state for accessing the database
#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub graph: Option<Arc<neo4rs::Graph>>,
    pub keyset: jsonwebtokens_cognito::KeySet,
    pub verifier: jsonwebtokens::Verifier,
}

/// Main async function run when executing the crate
#[tokio::main]
async fn main() {
    // Setup logger
    setup_logger().expect("Unable to setup logger with fern");

    // Connect to database
    let uri = match std::env::var("ENV") {
        Ok(val) => match val.as_str() {
            "DOCKER" => "database:7687",
            "ECS" => "database.advisory:7687",
            _ => "localhost:7687",
        },
        Err(_) => "localhost:7687",
    };
    let user = "neo4j";
    let pass = match std::env::var("DB_PASS") {
        Ok(val) => val,
        Err(_) => "test".to_string(),
    };
    let graph = Arc::new(neo4rs::Graph::new(uri, user, pass.as_str()).await.unwrap());
    let keyset = jsonwebtokens_cognito::KeySet::new("us-east-1", "us-east-1_Ye96rGbqV").unwrap();
    let verifier = keyset
        .new_id_token_verifier(&["5c6eva8nctpb3aug8l0teak36v"])
        .build()
        .unwrap();
    let state = SharedState {
        graph: Some(graph),
        keyset,
        verifier,
    };

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
    let https_port = 3000;

    // IP and Port to bind to
    let addr = match std::env::var("DOCKER") {
        Ok(_) => SocketAddr::from(([0, 0, 0, 0], https_port)),
        Err(_) => SocketAddr::from(([127, 0, 0, 1], https_port)),
    };
    log::info!("listening on {}", addr);

    // Bind axum app to configured IP and Port
    axum_server::bind_rustls(addr, config)
        .serve(app(state).into_make_service())
        .await
        .unwrap();
}

fn app(state: SharedState) -> Router {
    // Axum setup and configuration
    Router::new()
        // Add routes to specific handler functions
        .route("/health", get(get_health)) // Health check
        .route("/info", get(get_info))
        .route(
            "/people",
            delete(clear_people_handler).get(get_people_handler),
        )
        .route("/people/teacher", post(add_teacher_handler))
        .route("/people/student", post(add_student_handler))
        .route("/people/teacher/bulk", post(add_teacher_bulk))
        .route("/people/student/bulk", post(add_student_bulk))
        .route("/", put(get_advisories))
        // jsonwebtoken auth layer
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth::auth,
        ))
        // Add shared state to all requests
        .with_state(state)
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
