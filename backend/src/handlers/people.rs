use crate::SharedState;
use axum::{extract::Extension, http::StatusCode, Form, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Teacher {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Student {
    pub name: String,
    pub teachers: Vec<Teacher>,
    pub grade: Grade,
    pub sex: Option<Sex>,
}

impl Default for Student {
    fn default() -> Student {
        Self {
            name: "Default Name".to_string(),
            teachers: Vec::<Teacher>::new(),
            grade: Grade::Freshman,
            sex: Some(Sex::Male),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Grade {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Sex {
    Male,
    Female,
}

/// Handler to add a teacher, either a advisor or a student to the database
/// Uses [`Teacher`] as a form for input
#[axum_macros::debug_handler]
//TODO: actually add node to remote database
pub async fn add_teacher(
    Form(teacher): Form<Teacher>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Teacher>, StatusCode> {
    tracing::debug!("POST made to people/teacher");
    tracing::debug!("New teacher {:?} added", teacher);
    Ok(Json(teacher))
}

/// Handler to add a student, either a advisor or a student to the database
/// Uses [`Student`] as a form for input
#[axum_macros::debug_handler]
pub async fn add_student(
    Form(student): Form<Student>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Student>, StatusCode> {
    tracing::debug!("POST made to people/student");
    tracing::debug!("New student {:?} added", student);
    Ok(Json(student))
}
