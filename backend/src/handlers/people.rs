use crate::{
    database::{add_student, add_teacher, clear_people},
    forms::{
        student::StudentForm, students::StudentsForm, teacher::TeacherForm, teachers::TeachersForm,
        uid::UIDForm,
    },
    SharedState, Verify,
};
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

/// Handler to add a teacher to the database
///
/// Uses [`TeacherForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn clear_people_handler(
    Extension(state): Extension<Arc<SharedState>>,
    Json(form): Json<UIDForm>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("DELETE made to people");
    Ok(Json(
        clear_people(&state.graph, form.clone())
            .await
            .unwrap_or_else(|_| panic!("Unable to clear people for {}", form.uid)),
    ))
}

/// Handler to add a teacher to the database
///
/// Uses [`TeacherForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_teacher_handler(
    Extension(state): Extension<Arc<SharedState>>,
    Json(form): Json<TeacherForm>,
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
    Json(form): Json<TeachersForm>,
    Extension(state): Extension<Arc<SharedState>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/teacher/bulk");
    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    for teacher in form.0 {
        add_teacher(&state.graph, teacher).await?;
    }
    Ok(Json(1))
}

/// Handler to add a student to the database
///
/// Uses [`StudentForm`] as a form for input
#[axum_macros::debug_handler]
pub(crate) async fn add_student_handler(
    Extension(state): Extension<Arc<SharedState>>,
    Json(form): Json<StudentForm>,
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
    Json(form): Json<StudentsForm>,
    Extension(state): Extension<Arc<SharedState>>,
) -> Result<Json<u8>, StatusCode> {
    log::info!("POST made to people/student/bulk");
    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    for student in form.0 {
        add_student(&state.graph, student).await?;
    }
    Ok(Json(1))
}
