use crate::people::{Sex, Student, Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a person
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Person {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) user_id: String,
    /// Student's name - should be in `First Last` format, but can be anything that distinguishes them from other students
    pub(crate) name: String,
    /// Student's biological sex, represented by the [`Sex`] enum
    pub(crate) sex: Sex,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Student> for Person {
    fn from(s: Student) -> Self {
        Self {
            user_id: s.user_id,
            name: s.name,
            sex: s.sex,
        }
    }
}

impl From<Teacher> for Person {
    fn from(t: Teacher) -> Self {
        Self {
            user_id: t.user_id,
            name: t.name,
            sex: t.sex,
        }
    }
}
