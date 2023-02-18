use crate::{
    advisories::{AdvisoryGroup, Settings},
    auth::UserData,
    SharedState,
};
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
};

/// Wrapper of [`build_advisories`] called by https get requests to `/`
#[axum_macros::debug_handler]
pub(crate) async fn get_advisories(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Settings>,
) -> Result<Json<AdvisoryGroup>, StatusCode> {
    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(AdvisoryGroup::generate(form, graph, user.sub).await?)),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to get_advisories prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}
