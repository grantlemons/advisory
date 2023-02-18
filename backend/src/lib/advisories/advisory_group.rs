use crate::advisories::Advisory;
use crate::people::Teacher;
use crate::{advisories::Settings, lib::DatabaseNode, people::Student, Verify};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct AdvisoryGroup(pub(crate) Vec<Advisory>);

impl AdvisoryGroup {
    pub(crate) fn default(student_count: i16, advisory_count: i16) -> Self {
        let students_per_advisory = student_count / advisory_count;

        Self(vec![
            Advisory::default(students_per_advisory);
            advisory_count.try_into().unwrap()
        ])
    }

    pub(crate) fn populate_teachers(&mut self, teacher_pairs: &[[Option<Teacher>; 2]]) {
        for (index, target_advisory) in self.0.iter_mut().enumerate() {
            let [t1, t2] = teacher_pairs[index].clone();

            target_advisory.add_teacher(t1);
            target_advisory.add_teacher(t2);
        }
    }

    /// Places students into advisories and returns a vector of them
    ///
    /// Called by [`crate::advisories::Advisory`]
    pub(crate) async fn generate<T: Into<String> + Send>(
        form: Settings,
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<Self, StatusCode> {
        log::info!("Building advisories");
        if !form.verify() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // fetch student data from database
        let students: Vec<Student> = Student::get_nodes(graph, user_id).await?;

        // define constants for later use
        let student_count: i16 = students.len() as i16;
        let advisory_count: i16 = form.num_advisories;

        // create vector of advisories to fill
        let mut advisories: AdvisoryGroup = AdvisoryGroup::default(student_count, advisory_count);

        advisories.populate_teachers(&form.teacher_pairs);

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
