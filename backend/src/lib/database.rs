use crate::{
    forms::{delete::DeleteForm, student::StudentForm, teacher::TeacherForm},
    people::{grade::Grade, sex::Sex, student::Student, teacher::Teacher},
    Verify,
};
use axum::http::StatusCode;

pub(crate) async fn add_teacher(
    graph: &neo4rs::Graph,
    form: TeacherForm,
) -> Result<u8, StatusCode> {
    use neo4rs::query;

    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    log::info!("New teacher {:?} added", form.name);
    graph
        .run(
            query("CREATE (t:Teacher { name: $name, sex: $sex, user_id: $uid })")
                .param("name", form.name)
                .param("sex", form.sex.to_string())
                .param("uid", form.uid),
        )
        .await
        .unwrap();
    Ok(1)
}

pub(crate) async fn clear_people(
    graph: &neo4rs::Graph,
    form: DeleteForm,
) -> Result<u8, StatusCode> {
    use neo4rs::query;

    log::info!("Clearing all people for UID {}", form.uid);
    graph
        .run(query("MATCH (p { user_id: $uid }) DETACH DELETE p").param("uid", form.uid))
        .await
        .unwrap();
    Ok(1)
}

pub(crate) async fn add_student(
    graph: &neo4rs::Graph,
    form: StudentForm,
) -> Result<u8, StatusCode> {
    use neo4rs::query;

    if !form.verify() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    log::info!("New student {:?} added", form.name);
    let teacher_names: Vec<String> = form.teachers.iter().map(|t| t.name.clone()).collect();
    graph
        .run(
            query("CREATE (s:Student { name: $name, sex: $sex, grade: $grade, user_id: $uid })")
                .param("name", String::from(&form.name))
                .param("sex", form.sex.to_string())
                .param("grade", i64::from(form.clone().grade))
                .param("uid", String::from(&form.uid)),
        )
        .await
        .expect("Unable to send query to database");
    graph
        .run(
            query(
                "MATCH (t:Teacher {user_id: $uid}), (s:Student { name: $name, sex: $sex, grade: $grade, user_id: $uid }) \
                WHERE t.name in $t_arr \
                CREATE (t)-[:TEACHES]->(s) \
                RETURN t, s",
            )
            .param("t_arr", teacher_names)
            .param("name", String::from(&form.name))
            .param("sex", form.sex.to_string())
            .param("grade", i64::from(form.grade))
            .param("uid", String::from(&form.uid)),
        )
        .await
        .expect("Unable to send query to database");
    Ok(1)
}

/// Helper function for [`crate::advisories::builder::build_advisories`] to get vector of students from neo4j database using [`neo4rs`]
pub(crate) async fn get_students(
    graph: &neo4rs::Graph,
    uid: &str,
) -> Result<Vec<Student>, StatusCode> {
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

/// Helper function for [`crate::advisories::builder::build_advisories`] to get vector of teachers from neo4j database using [`neo4rs`]
pub(crate) async fn get_teachers(
    graph: &neo4rs::Graph,
    uid: &str,
) -> Result<Vec<Teacher>, StatusCode> {
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