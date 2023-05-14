use crate::{auth::UserData, SharedState};
use advisory_backend_lib::{
    advisories::{Organization, Settings},
    people::Student,
    DatabaseNode, Verify,
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
) -> Result<Json<Organization>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => {
                let students: Vec<Student> = Student::get_nodes(graph, user.user_id()).await?;
                Ok(Json(Organization::generate(&form, students).await?))
            }
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to get_advisories prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}
