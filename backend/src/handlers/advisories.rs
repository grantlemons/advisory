use crate::{
    advisory::advisory::Advisory,
    forms::advisory::AdvisoryForm,
    people::{grade::Grade, sex::Sex, student::Student, teacher::Teacher},
    SharedState, Verify,
};
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

/// Wrapper of [`build_advisories`] called by https get requests to `/`
#[axum_macros::debug_handler]
pub(crate) async fn get_advisories(
    Json(form): Json<AdvisoryForm>,
    state: Extension<Arc<SharedState>>,
) -> Result<Json<Vec<Advisory>>, StatusCode> {
    log::info!("GET made to get_advisories");
    Ok(Json(
        build_advisories(&state.graph, form)
            .await
            .expect("Unable to build advisories"),
    ))
}

/// Places students into advisories and returns a vector of them
///
/// Called by [`get_advisories`]
pub(crate) async fn build_advisories(
    graph: &neo4rs::Graph,
    form: AdvisoryForm,
) -> Result<Vec<Advisory>, StatusCode> {
    log::info!("Building advisories");
    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // create vectors from data from database
    let students: Vec<Student> = get_students(graph, form.uid.as_str()).await?;
    let mut teachers: Vec<Teacher> = get_teachers(graph, form.uid.as_str()).await?;

    // create vector of advisories to fill
    let s: i16 = students.len() as i16;
    let a: i16 = form.num_advisories;
    log::info!("{} Students, {} Advisories", s, a);
    let mut advisories: Vec<Advisory> = vec![Advisory::default(s / a); a.try_into().unwrap()];

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

/// Helper function for [`build_advisories`] to get vector of students from neo4j database using [`neo4rs`]
async fn get_students(graph: &neo4rs::Graph, uid: &str) -> Result<Vec<Student>, StatusCode> {
    log::info!("Getting students from database");
    use neo4rs::*;

    // Get the result of a Cypher query to the neo4j database
    let mut result = match graph
        .execute(
            query(
                "MATCH (s:Student { user_id: $UID })<-[:TEACHES]-(t) \
                RETURN \
                distinct(s) as students, \
                collect(t) as teachers",
            )
            .param("UID", uid),
        )
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_GATEWAY),
    };

    // Create and initialize returned vector
    let mut students: Vec<Student> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        // Get student data from returned row of the database query
        let student: Node = row.get("students").unwrap();
        let name: String = student.get("name").unwrap();
        let grade: Grade = Grade::from(student.get::<i64>("grade").unwrap());
        let sex: Option<Sex> = Some(Sex::from(student.get::<String>("sex").unwrap()));

        log::info!(
            "Student data is {{name: {}, grade: {}, sex: {:?}}}",
            name,
            grade,
            sex
        );

        // Get the student's teachers
        log::info!("Getting {}'s teachers", name);
        let mut t_structs: Vec<Teacher> = Vec::new();
        match row.get::<Vec<Node>>("teachers") {
            Some(teachers) => {
                t_structs = teachers
                    .into_iter()
                    .map(|t| Teacher {
                        name: t.get("name").unwrap(),
                        sex: Sex::from(t.get::<String>("sex").unwrap()),
                    })
                    .collect();
            }
            None => {
                log::error!("No teachers!")
            }
        }

        // Add student with all fields to the students vector
        let student = Student {
            name,
            teachers: t_structs,
            grade,
            sex,
        };
        log::info!("Adding {} to students vector", student);
        students.push(student)
    }
    log::info!("Done getting students!");
    Ok(students)
}

/// Helper function for [`build_advisories`] to get vector of teachers from neo4j database using [`neo4rs`]
async fn get_teachers(graph: &neo4rs::Graph, uid: &str) -> Result<Vec<Teacher>, StatusCode> {
    log::info!("Getting teachers from database");
    use neo4rs::*;

    // Get the result of a Cypher query to the neo4j database
    let mut result = match graph
        .execute(
            query(
                "MATCH (t:Teacher { user_id: $UID }) \
                RETURN distinct(t) as teachers",
            )
            .param("UID", uid),
        )
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_GATEWAY),
    };

    // Create and initialize returned vector
    let mut teachers: Vec<Teacher> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        // Get teacher data from returned row of the database query
        let teacher: Node = row.get("teachers").unwrap();
        let name: String = teacher.get("name").unwrap();
        let sex: Sex = Sex::from(teacher.get::<String>("sex").unwrap());

        log::info!("Teacher data is {{name: {}, sex: {:?}}}", name, sex);

        // Add teacher will all fields to the teachers vector
        let teacher = Teacher { name, sex };
        log::info!("Adding {} to teacher vector", teacher);
        teachers.push(teacher)
    }
    log::info!("Done getting teachers!");
    Ok(teachers)
}
