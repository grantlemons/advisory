use crate::forms::teacher::TeacherForm;
use serde::{Deserialize, Serialize};

/// Form used for post requests to people/student/bulk
#[derive(Deserialize, Serialize, Clone)]
pub struct TeachersForm(pub(crate) Vec<TeacherForm>);

impl crate::Verify for TeachersForm {
    fn verify(&self) -> bool {
        // Check if each teacher is valid
        let mut teachers_valid = true;
        for i in &self.0 {
            teachers_valid = teachers_valid && i.verify();
        }
        teachers_valid
    }
}
