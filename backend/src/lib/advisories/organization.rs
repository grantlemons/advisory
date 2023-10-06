use crate::{
    advisories::{Advisory, Settings},
    people::{Student, Teacher},
    Verify,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Multiple advisories make up an organization
/// Generating this struct is the goal of the program
#[derive(Deserialize, Serialize)]
pub struct Organization(pub Arc<[Advisory]>);

impl From<Arc<[Advisory]>> for Organization {
    fn from(value: Arc<[Advisory]>) -> Self {
        Self(value)
    }
}

impl From<Vec<Advisory>> for Organization {
    fn from(value: Vec<Advisory>) -> Self {
        let arc: Arc<[Advisory]> = value.into();
        arc.into()
    }
}

impl Organization {
    /// Allocate a vector of advisories of the appropriate size for the number of students and
    /// advisories
    fn allocate_advisories(student_count: u16, advisory_count: u16) -> Vec<Advisory> {
        vec![Advisory::new(student_count / advisory_count); advisory_count as usize]
    }

    /// Assign teachers to advisories in accordance with the groupings passed in
    fn assign_teachers(advisories: &mut [Advisory], teacher_groupings: Arc<[Arc<[Teacher]>]>) {
        for (index, target_advisory) in advisories.iter_mut().enumerate() {
            teacher_groupings[index]
                .iter()
                .for_each(|t| target_advisory.add_teacher(t.clone()));
        }
    }

    /// Places students into advisories and returns a vector of them
    pub async fn generate(form: &Settings, students: Arc<[Student]>) -> Result<Self, StatusCode> {
        log::trace!("Building advisories");
        form.verify()?;

        // define values for later use
        let student_count: u16 = students.len() as u16;
        let advisory_count: u16 = form.num_advisories;

        // create vector of advisories to fill
        let mut advisories: Vec<Advisory> =
            Organization::allocate_advisories(student_count, advisory_count);

        Organization::assign_teachers(&mut advisories, form.teacher_groupings.clone());

        // add students to advisories
        for student in students.iter() {
            let max: Option<usize> = advisories
                .iter()
                .map(|target_advisory| {
                    target_advisory.calculate_weight(
                        student,
                        &form.weights,
                        student_count / advisory_count,
                    )
                })
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(index, _)| index);
            if let Some(max) = max {
                advisories[max].add_student(student.clone());
            }
        }

        Ok(advisories.into())
    }
}
