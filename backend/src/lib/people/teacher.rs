use crate::people::sex::Sex;
use serde::{Deserialize, Serialize};

/// Representation of a teacher
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Teacher {
    pub(crate) name: String,
    pub(crate) sex: Sex,
}

impl crate::Verify for Teacher {
    fn verify(&self) -> bool {
        !self.name.is_empty()
    }
}

impl std::fmt::Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
