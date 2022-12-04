use crate::people::{grade::Grade, sex::Sex, teacher::Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a student
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Student {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) user_id: String,
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
            user_id: String::from("USER_ID"),
            name: String::from("Default Name"),
            teachers: Vec::<Teacher>::new(),
            grade: Grade::Freshman,
            sex: Some(Sex::Male),
        }
    }
}
