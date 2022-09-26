use crate::people::{grade::Grade, sex::Sex, teacher::Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a student
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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
