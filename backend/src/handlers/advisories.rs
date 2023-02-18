use crate::{auth::UserData, SharedState};
use advisory_backend_lib::{
    advisories::{AdvisoryGroup, Settings},
    Verify,
};
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
};

/// Get list of populated advisories based around passed settings and database values
#[axum_macros::debug_handler]
pub(crate) async fn get_advisories(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Settings>,
) -> Result<Json<AdvisoryGroup>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => Ok(Json(
                AdvisoryGroup::generate(form, graph, user.user_id()).await?,
            )),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to get_advisories prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}
