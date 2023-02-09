use crate::{
    advisories::Advisory,
    database::{get_students, get_teachers},
    people::{Student, Teacher},
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

    // create vectors from data from database
    let students: Vec<Student> = get_students(user.clone(), graph).await?;
    let mut teachers: Vec<Teacher> = get_teachers(user.clone(), graph).await?;

    // create vector of advisories to fill
    let s: i16 = students.len() as i16;
    let a: i16 = form.num_advisories;
    log::info!("{} Students, {} Advisories", s, a);
    let mut advisories: Vec<Advisory> =
        vec![Advisory::default(s / a, user.sub); a.try_into().unwrap()];

    // add teachers to advisories
    for i in &mut advisories {
        let t1 = teachers.pop();
        let t2 = teachers.pop();
        log::info!("Adding {:?} to {}", vec![&t1, &t2], i);
        i.add_teacher(t1);
        i.add_teacher(t2);
    }
    // add students to advisories
    for i in students {
        let max: Option<usize> = advisories
            .iter()
            .map(|x| {
                log::info!("Calculating weight for {} & {}", i, x);
                let weight = (form.weights.has_teacher as i32
                    * x.has_teacher(&i) as i32
                    * (s / a) as i32)
                    + (form.weights.sex_diverse as i32 * x.get_remaining_sex(&i.sex) as i32)
                    + (form.weights.grade_diverse as i32 * x.get_remaining_grade(&i.grade) as i32);
                log::info!("Weight for {} & ({}) is {}", i, x, weight);
                weight
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);
        if let Some(max) = max {
            log::info!("Adding {} to {}", i, advisories[max]);
            advisories[max].add_student(i);
        }
    }
    log::info!("build_advisories complete");
    Ok(advisories)
}
