use crate::people::sex::Sex;
use serde::{Deserialize, Serialize};

/// Representation of a teacher
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Teacher {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) user_id: String,
    /// Teacher's name - should be in `First Last` format, but can be anything that distinguishes them from other teachers
    pub(crate) name: String,
    /// Student's biological sex, represented by the [`Sex`] enum
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
