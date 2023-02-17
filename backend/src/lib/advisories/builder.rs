use crate::{advisories::Advisory, database::get_students, people::Student, Settings, Verify};
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

    // create vectors from data from database
    let students: Vec<Student> = get_students(user.clone(), graph).await?;

    // create vector of advisories to fill
    let num_students: i16 = students.len() as i16;
    let num_advisories: i16 = form.num_advisories;
    let students_per_advisory: i16 = num_students / num_advisories;
    log::info!("{} Students, {} Advisories", num_students, num_advisories);
    let mut advisories: Vec<Advisory> =
        vec![Advisory::default(students_per_advisory); num_advisories.try_into().unwrap()];

    form.populate_advisories(&mut advisories);

    let number_of_sexes = 2;
    let number_of_grades = 4;

    // add students to advisories
    for student in students {
        let max: Option<usize> = advisories
            .iter()
            .map(|target_advisory| {
                log::info!("Calculating weight for {} & {}", student, target_advisory);
                let teacher_weighted_value = form.weights.has_teacher as i32
                    * students_per_advisory as i32
                    * target_advisory.has_teacher(&student) as i32;
                let sexes_weighted_value = number_of_sexes
                    * (form.weights.sex_diverse as i32
                        * target_advisory.get_remaining_sex(&student.sex) as i32);
                let grade_weighted_value = number_of_grades
                    * (form.weights.grade_diverse as i32
                        * target_advisory.get_remaining_grade(&student.grade) as i32);
                let weighted_value =
                    teacher_weighted_value + sexes_weighted_value + grade_weighted_value;
                log::info!(
                    "Weights for {} and {} is {} ({}, {}, {})",
                    student,
                    target_advisory,
                    weighted_value,
                    teacher_weighted_value,
                    sexes_weighted_value,
                    grade_weighted_value
                );
                weighted_value
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);
        if let Some(max) = max {
            log::info!("Adding {} to {}", student, advisories[max]);
            advisories[max].add_student(student);
        }
    }
    log::info!("build_advisories complete");
    Ok(advisories)
}
