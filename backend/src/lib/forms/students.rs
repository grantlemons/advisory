use crate::forms::student::StudentForm;
use serde::{Deserialize, Serialize};

/// Form used for post requests to people/student/bulk
#[derive(Deserialize, Serialize, Clone)]
pub struct StudentsForm(pub(crate) Vec<StudentForm>);

impl crate::Verify for StudentsForm {
    fn verify(&self) -> bool {
        // Check if each teacher is valid
        let mut students_valid = true;
        for i in &self.0 {
            students_valid = students_valid && i.verify();
        }
        students_valid
    }
}
