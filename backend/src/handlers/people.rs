use crate::SharedState;
use axum::{extract::Extension, http::StatusCode, Form, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Representation of a teacher
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Teacher {
    pub(crate) name: String,
    pub(crate) sex: Option<Sex>,
}

impl std::fmt::Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Representation of a teacher
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Student {
    /// Student's name - should be in `First Last` format, but can be anything that distinguishes them from other students
    pub(crate) name: String,
    /// Vector list of the student's teacher for the current academic school year
    pub(crate) teachers: Vec<Teacher>,
    /// Student's grade represented with the [`Grade`] enum
    pub(crate) grade: Grade,
    /// Student's biological sex, represented by the optional [`Sex`] enum
    pub(crate) sex: Option<Sex>,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Default values of the [`Student`] struct
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

/// Representaion of possible grades for students
///
/// Adding more options requires changing the grade "spots" tuple in [`super::advisories::Advisory`] as well as adding the mapping to the implementations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Grade {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

/// Mapping for string to [`Grade`] enum used for parsing info from database
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

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Freshman => "Freshman",
            Self::Sophomore => "Sophomore",
            Self::Junior => "Junior",
            Self::Senior => "Senior",
        };
        write!(f, "{}", string)
    }
}

/// Representaion of possible sexes for students within database
///
/// Adding more options requires changing the sex "spots" tuple in [`super::advisories::Advisory`] as well as adding the mapping to the implementations.
///
/// I understand that grouping it like this might be somewhat sensitive, but it is needed for attempting diversity in the advisories. Sex is used in place of gender to avoid
/// complexities and ambiguity by representing biological sex. I know that there are some exceptions, but there is no pressing need to accommodate that edge case currently.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Sex {
    Male,
    Female,
}

/// Mapping for string to [`Sex`] enum used for parsing info from database
impl From<String> for Sex {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Male" => Self::Male,
            "Female" => Self::Female,
            _ => panic!("{} not in list of sexes", s),
        }
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Male => "Male",
            Self::Female => "Female",
        };
        write!(f, "{}", string)
    }
}

/// Handler to add a teacher, either a advisor or a student to the database
///
/// Uses [`Teacher`] as a form for input
//TODO: actually add node to remote database
pub(crate) async fn add_teacher(
    Form(teacher): Form<Teacher>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Teacher>, StatusCode> {
    log::debug!("POST made to people/teacher");
    log::debug!("New teacher {:?} added", teacher);
    Ok(Json(teacher))
}

/// Handler to add a student, either a advisor or a student to the database
///
/// Uses [`Student`] as a form for input
pub(crate) async fn add_student(
    Form(student): Form<Student>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Student>, StatusCode> {
    log::debug!("POST made to people/student");
    log::debug!("New student {:?} added", student);
    Ok(Json(student))
}
