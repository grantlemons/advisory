use crate::{
    advisories::{builder::build_advisories, Advisory},
    Settings, SharedState,
};
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

/// Wrapper of [`build_advisories`] called by https get requests to `/`
#[axum_macros::debug_handler]
pub(crate) async fn get_advisories(
    Json(form): Json<Settings>,
    state: Extension<Arc<SharedState>>,
) -> Result<Json<Vec<Advisory>>, StatusCode> {
    log::info!("GET made to get_advisories");
    Ok(Json(
        build_advisories(&state.graph, form)
            .await
            .expect("Unable to build advisories"),
    ))
}
