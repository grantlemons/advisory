use crate::{
    forms::teacher::TeacherForm,
    people::{grade::Grade, sex::Sex},
};
use serde::{Deserialize, Serialize};

/// Form used for post requests to people/student
#[derive(Deserialize, Serialize, Clone)]
pub struct StudentForm {
    pub(crate) name: String,
    pub(crate) teachers: Vec<TeacherForm>,
    pub(crate) sex: Sex,
    pub(crate) grade: Grade,
    pub(crate) uid: String,
}

impl crate::Verify for StudentForm {
    fn verify(&self) -> bool {
        // Check if each teacher is valid
        let mut teachers_valid = true;
        for i in &self.teachers {
            teachers_valid = teachers_valid && i.verify()
        }
        !self.name.is_empty() && teachers_valid && !self.uid.is_empty()
    }
}
