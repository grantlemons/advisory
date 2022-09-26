use crate::people::sex::Sex;
use serde::{Deserialize, Serialize};

/// Form used for post requests to people/teacher
#[derive(Deserialize, Serialize, Clone)]
pub struct TeacherForm {
    pub(crate) name: String,
    pub(crate) sex: Sex,
    pub(crate) uid: String,
}

impl crate::Verify for TeacherForm {
    fn verify(&self) -> bool {
        !self.name.is_empty() && !self.uid.is_empty()
    }
}
