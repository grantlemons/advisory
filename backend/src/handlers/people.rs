use crate::{
    database::{add_student, add_teacher, clear_people},
    people::{Student, Teacher},
    SharedState, UserIDForm, Verify,
};
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

/// Handler to add a teacher to the database
///
/// Uses [`TeacherForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn clear_people_handler(
    Extension(state): Extension<Arc<SharedState>>,
    Json(form): Json<UserIDForm>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("DELETE made to people");
    Ok(Json(
        clear_people(&state.graph, form.clone())
            .await
            .unwrap_or_else(|_| panic!("Unable to clear people for {}", form.user_id)),
    ))
}

/// Handler to add a teacher to the database
///
/// Uses [`Teacher`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_handler(
    Extension(state): Extension<Arc<SharedState>>,
    Json(form): Json<Teacher>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/teacher");
    Ok(Json(
        add_teacher(&state.graph, form)
            .await
            .expect("Unable to add teacher"),
    ))
}

/// Handler to add many teachers
///
/// Uses [`TeachersForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_bulk(
    Json(forms): Json<Vec<Teacher>>,
    Extension(state): Extension<Arc<SharedState>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/teacher/bulk");
    if !forms.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    for teacher in forms {
        add_teacher(&state.graph, teacher).await?;
    }
    Ok(Json(1))
}

/// Handler to add a student to the database
///
/// Uses [`StudentForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_handler(
    Json(form): Json<Student>,
    Extension(state): Extension<Arc<SharedState>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/student");
    Ok(Json(
        add_student(&state.graph, form)
            .await
            .expect("Unable to add student"),
    ))
}

/// Handler to add many students
///
/// Uses [`StudentsForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_bulk(
    Json(forms): Json<Vec<Student>>,
    Extension(state): Extension<Arc<SharedState>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/student/bulk");
    if !forms.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    for student in forms {
        add_student(&state.graph, student).await?;
    }
    Ok(Json(1))
}
