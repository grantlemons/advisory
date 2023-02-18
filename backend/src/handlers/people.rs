use crate::{
    auth::UserData,
    database::{add_student, add_teacher, clear_people, get_people},
    people::{Person, Student, Teacher},
    SharedState, Verify,
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
    log::info!("DELETE made to people");

    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(clear_people(&user, graph).await.unwrap_or_else(
                |_| panic!("Unable to clear people for {} ({})", user.sub, user.sub),
            ))),
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
    log::info!("GET made to people");

    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(get_people(&user, graph).await.unwrap_or_else(|_| {
                panic!("Unable to get people for {} ({})", user.sub, user.sub)
            }))),
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
    log::info!("POST made to people/teacher");

    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(
                add_teacher(&user, graph, form)
                    .await
                    .expect("Unable to add teacher"),
            )),
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
    log::info!("POST made to people/teacher/bulk");

    if let Some(user) = user_option {
        if !form.verify() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        match &state.graph {
            Some(graph) => {
                crate::database::add_teacher_bulk(&user, graph, form).await?;
                Ok(Json(1))
            }
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
    log::info!("POST made to people/student");

    if let Some(user) = user_option {
        match &state.graph {
            Some(graph) => Ok(Json(
                add_student(&user, graph, form)
                    .await
                    .expect("Unable to add student"),
            )),
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
    log::info!("POST made to people/student/bulk");

    if let Some(user) = user_option {
        if !form.verify() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        match &state.graph {
            Some(graph) => {
                crate::database::add_student_bulk(&user, graph, form).await?;
                Ok(Json(1))
            }
            None => Err(StatusCode::BAD_GATEWAY),
        }
    } else {
        log::info!("Unauthorized access to add_student_bulk prevented");
        Err(StatusCode::UNAUTHORIZED)
    }
}
