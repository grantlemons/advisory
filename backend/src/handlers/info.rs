use axum::Json;
use serde::Serialize;

/// Healthcheck handler
///
/// Returns `Healthy!` if healthy
pub(crate) async fn get_health() -> &'static str {
    "Healthy!"
}

/// Crate information handler used to get information on the server
///
/// Uses [`CrateInfo`] struct
pub(crate) async fn get_info() -> Json<CrateInfo> {
    Json(CrateInfo {
        name: env!("CARGO_PKG_NAME"),
        authors: env!("CARGO_PKG_AUTHORS").split(',').collect(),
        version: env!("CARGO_PKG_VERSION"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        license: env!("CARGO_PKG_LICENSE"),
        repository: env!("CARGO_PKG_REPOSITORY"),
    })
}

/// Information on version and other fields set in the cargo manifest
#[derive(Debug, Serialize, PartialEq, Eq)]
pub(crate) struct CrateInfo {
    pub(crate) name: &'static str,
    pub(crate) authors: Vec<&'static str>,
    pub(crate) version: &'static str,
    pub(crate) description: &'static str,
    pub(crate) license: &'static str,
    pub(crate) repository: &'static str,
}
