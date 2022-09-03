use crate::SharedState;
use axum::{extract::Extension, http::StatusCode, Form, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Teacher {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, merge::Merge)]
pub struct Student {
    #[merge(skip)]
    pub name: String,
    #[merge(strategy = merge::vec::append)]
    pub teachers: Vec<Teacher>,
    #[merge(skip)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Grade {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

impl From<i64> for Grade {
    fn from(n: i64) -> Self {
        match n {
            9 => Self::Freshman,
            10 => Self::Sophomore,
            11 => Self::Junior,
            12 => Self::Senior,
            _ => panic!("Grade must be from 9-12"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}

impl From<String> for Sex {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Male" => Self::Male,
            "Female" => Self::Female,
            _ => panic!("{} not in list of sexes", s),
        }
    }
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
