use crate::{
    advisories::{Advisory, Settings},
    people::{Student, Teacher},
    Verify,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

/// Multiple advisories make up an organization
/// Generating this struct is the goal of the program
#[derive(Deserialize, Serialize)]
pub struct Organization(pub Vec<Advisory>);

impl Organization {
    /// Initialize [`Organization`] to the number of desired advisories
    /// Initialize each advisory with quotas
    fn new(student_count: i16, advisory_count: i16) -> Self {
        let students_per_advisory = student_count / advisory_count;

        Self(vec![
            Advisory::new(students_per_advisory);
            advisory_count as usize
        ])
    }

    /// Assign teachers to advisories in accordance with the groupings passed in
    fn assign_teachers(&mut self, teacher_groupings: &[Vec<Teacher>]) {
        for (index, target_advisory) in self.0.iter_mut().enumerate() {
            teacher_groupings[index]
                .iter()
                .for_each(|t| target_advisory.add_teacher(t.clone()));
        }
    }

    /// Places students into advisories and returns a vector of them
    pub async fn generate(form: Settings, students: Vec<Student>) -> Result<Self, StatusCode> {
        log::trace!("Building advisories");
        form.verify()?;

        // define values for later use
        let student_count: i16 = students.len() as i16;
        let advisory_count: i16 = form.num_advisories;

        // create vector of advisories to fill
        let mut advisories: Organization = Organization::new(student_count, advisory_count);

        advisories.assign_teachers(&form.teacher_groupings);

        // add students to advisories
        for student in students {
            let max: Option<usize> = advisories
                .0
                .iter()
                .map(|target_advisory| {
                    target_advisory.calculate_weight(
                        &student,
                        &form.weights,
                        student_count / advisory_count,
                    )
                })
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(index, _)| index);
            if let Some(max) = max {
                advisories.0[max].add_student(student);
            }
        }
        Ok(advisories)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_group() {
        let advisory_group = Organization::new(10, 5);
        assert_eq!(advisory_group.0.len(), 5);
    }

    #[test]
    fn assign_teachers_to_group() {
        let teacher_groupings: &[Vec<Teacher>] = &[vec![Teacher::default(); 2]];

        let mut advisory_group = Organization::new(10, 1);
        advisory_group.assign_teachers(teacher_groupings);

        assert_eq!(advisory_group.0.len(), 1);
    }
}
