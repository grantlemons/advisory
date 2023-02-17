use crate::{
    advisories::Advisory, database::get_students, lib::advisories::AdvisoryGroup, people::Student,
    Settings, Verify,
};
use axum::http::StatusCode;

/// Places students into advisories and returns a vector of them
///
/// Called by [`crate::advisories::Advisory`]
pub(crate) async fn build_advisories(
    user: crate::auth::UserData,
    graph: &neo4rs::Graph,
    form: Settings,
) -> Result<Vec<Advisory>, StatusCode> {
    log::info!("Building advisories");
    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // fetch student data from database
    let students: Vec<Student> = get_students(&user, graph).await?;

    // define constants for later use
    let student_count: i16 = students.len() as i16;
    let advisory_count: i16 = form.num_advisories;

    // create vector of advisories to fill
    log::info!("{} Students, {} Advisories", student_count, advisory_count);
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
            log::info!("Adding {} to {}", student, advisories.0[max]);
            advisories.0[max].add_student(student);
        }
    }
    log::info!("build_advisories complete");
    Ok(advisories.0)
}
