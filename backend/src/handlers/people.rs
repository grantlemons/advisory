use crate::{auth::UserData, SharedState};
use advisory_backend_lib::{
    people::{Person, Student, Teacher},
    DatabaseNode, Verify,
};
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
};

/// Handler to clear all people for a specific user
#[axum_macros::debug_handler]
pub(crate) async fn clear_people_handler(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
) -> Result<Json<u8>, StatusCode> {
    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(Person::clear_nodes(graph, user.sub).await?)),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to clear_people_handler prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Handler to get all people for a specific user
#[axum_macros::debug_handler]
pub(crate) async fn get_people_handler(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
) -> Result<Json<Vec<Person>>, StatusCode> {
    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(Person::get_nodes(graph, user.sub).await?)),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to get_people_handler prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Handler to add a teacher to the database
///
/// Uses [`Teacher`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_handler(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Teacher>,
) -> Result<Json<u8>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => Ok(Json(form.add_node(graph, user.sub, true).await?)),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to add_teacher_handler prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Handler to add many teachers
///
/// Uses a vector of [`Teacher`]s as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_bulk(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Vec<Teacher>>,
) -> Result<Json<u8>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => Ok(Json(
                Teacher::add_multiple_nodes(form, graph, user.sub, true).await?,
            )),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to add_teacher_bulk prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Handler to add a student to the database
///
/// Uses [`Student`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_handler(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Student>,
) -> Result<Json<u8>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => Ok(Json(form.add_node(graph, user.sub, true).await?)),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to add_student_handler prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Handler to add many students
///
/// Uses a vector of [`Student`]s as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_bulk(
    State(state): State<SharedState>,
    Extension(user_option): Extension<Option<UserData>>,
    Json(form): Json<Vec<Student>>,
) -> Result<Json<u8>, StatusCode> {
    if let Some(user) = user_option {
        form.verify()?;
        match &state.graph {
            Some(graph) => Ok(Json(
                Student::add_multiple_nodes(form, graph, user.sub, true).await?,
            )),
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to add_student_bulk prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}
