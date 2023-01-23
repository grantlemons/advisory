use crate::{
    database::{add_student, add_teacher, clear_people, get_people},
    people::{Person, Student, Teacher},
    SharedState, UserIDForm, Verify,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

/// Handler to clear all people for a specific user
///
/// Uses [`UserIDForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn clear_people_handler(
    State(state): State<SharedState>,
    Json(form): Json<UserIDForm>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("DELETE made to people");
    match &state.graph {
        Some(graph) => Ok(Json(
            clear_people(graph, form.clone())
                .await
                .unwrap_or_else(|_| panic!("Unable to clear people for {}", form.user_id)),
        )),
        None => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Handler to get all people for a specific user
///
/// Uses [`UserIDForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn get_people_handler(
    State(state): State<SharedState>,
    Json(form): Json<UserIDForm>,
) -> Result<Json<Vec<Person>>, StatusCode> {
    log::info!("GET made to people");
    match &state.graph {
        Some(graph) => {
            Ok(Json(get_people(graph, form.clone()).await.unwrap_or_else(
                |_| panic!("Unable to get people for {}", form.user_id),
            )))
        }
        None => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Handler to add a teacher to the database
///
/// Uses [`Teacher`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_handler(
    State(state): State<SharedState>,
    Json(form): Json<Teacher>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/teacher");
    match &state.graph {
        Some(graph) => Ok(Json(
            add_teacher(graph, form)
                .await
                .expect("Unable to add teacher"),
        )),
        None => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Handler to add many teachers
///
/// Uses a vector of [`Teacher`]s as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_bulk(
    State(state): State<SharedState>,
    Json(forms): Json<Vec<Teacher>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/teacher/bulk");
    if !forms.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    match &state.graph {
        Some(graph) => {
            for teacher in forms {
                add_teacher(graph, teacher).await?;
            }
            Ok(Json(1))
        }
        None => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Handler to add a student to the database
///
/// Uses [`Student`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_handler(
    State(state): State<SharedState>,
    Json(form): Json<Student>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/student");
    match &state.graph {
        Some(graph) => Ok(Json(
            add_student(graph, form)
                .await
                .expect("Unable to add student"),
        )),
        None => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Handler to add many students
///
/// Uses a vector of [`Student`]s as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_bulk(
    State(state): State<SharedState>,
    Json(forms): Json<Vec<Student>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/student/bulk");
    if !forms.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    match &state.graph {
        Some(graph) => {
            for student in forms {
                add_student(graph, student).await?;
            }
            Ok(Json(1))
        }
        None => Err(StatusCode::BAD_GATEWAY),
    }
}
