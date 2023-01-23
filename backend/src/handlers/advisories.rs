use crate::{
    advisories::{builder::build_advisories, Advisory},
    Settings, SharedState,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

/// Wrapper of [`build_advisories`] called by https get requests to `/`
#[axum_macros::debug_handler]
pub(crate) async fn get_advisories(
    State(state): State<SharedState>,
    Json(form): Json<Settings>,
) -> Result<Json<Vec<Advisory>>, StatusCode> {
    log::info!("GET made to get_advisories");
    match &state.graph {
        Some(graph) => Ok(Json(
            build_advisories(graph, form)
                .await
                .expect("Unable to build advisories"),
        )),
        None => Err(StatusCode::BAD_GATEWAY),
    }
}
