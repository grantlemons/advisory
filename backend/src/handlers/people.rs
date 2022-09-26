use crate::{
    forms::{
        student::StudentForm, students::StudentsForm, teacher::TeacherForm, teachers::TeachersForm,
    },
    SharedState, Verify,
};
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

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

pub(crate) async fn add_teacher(
    graph: &neo4rs::Graph,
    form: TeacherForm,
) -> Result<u8, StatusCode> {
    use neo4rs::query;

    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    log::info!("New teacher {:?} added", form.name);
    graph
        .run(
            query("CREATE (t:Teacher { name: $name, sex: $sex, user_id: $uid })")
                .param("name", String::from(form.name))
                .param("sex", form.sex.to_string())
                .param("uid", String::from(form.uid)),
        )
        .await
        .unwrap();
    Ok(1)
}

pub(crate) async fn add_student(
    graph: &neo4rs::Graph,
    form: StudentForm,
) -> Result<u8, StatusCode> {
    use neo4rs::query;

    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    log::info!("New student {:?} added", form.name);
    let teacher_names: Vec<String> = form
        .teachers
        .iter()
        .map(|t| format!("{}", t.name.clone()))
        .collect();
    graph
        .run(
            query("CREATE (s:Student { name: $name, sex: $sex, user_id: $uid })")
                .param("name", String::from(&form.name))
                .param("sex", form.sex.to_string())
                .param("uid", String::from(&form.uid)),
        )
        .await
        .expect("Unable to send query to database");
    graph
        .run(
            query(
                "MATCH (t:Teacher {user_id: $uid}), (s:Student { name: $name, sex: $sex, user_id: $uid }) \
                WHERE t.name in $tarr \
                CREATE (t)-[:TEACHES]->(s) \
                RETURN t, s",
            )
            .param("tarr", teacher_names)
            .param("name", String::from(&form.name))
            .param("sex", form.sex.to_string())
            .param("uid", String::from(&form.uid)),
        )
        .await
        .expect("Unable to send query to database");
    Ok(1)
}
