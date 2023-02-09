use crate::people::{Student, Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a person
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Person {
    /// Student's name - should be in `First Last` format, but can be anything that distinguishes them from other students
    pub(crate) name: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Student> for Person {
    fn from(s: Student) -> Self {
        Self { name: s.name }
    }
}

impl From<Teacher> for Person {
    fn from(t: Teacher) -> Self {
        Self { name: t.name }
    }
}
