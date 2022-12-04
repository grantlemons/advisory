use crate::advisories::weights::Weights;
use crate::people::{grade::Grade, sex::Sex};
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::advisory::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct AdvisoryForm {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) uid: String,
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub(crate) weights: Weights,
    /// Number of advisories to be generated
    pub(crate) num_advisories: i16,
}

impl crate::Verify for AdvisoryForm {
    fn verify(&self) -> bool {
        !self.uid.is_empty() && self.weights.verify() && self.num_advisories > 0
    }
}

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

/// Form for [`crate::database::clear_people`]'s input
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct UIDForm {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) uid: String,
}
