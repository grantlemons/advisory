use super::people::*;
use crate::{SharedState, Verify};
use axum::{extract::Extension, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Representation of an advisory
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Advisory {
    /// Vector of [`Teacher`] structs
    advisors: Vec<Teacher>,
    /// Vector of [`Student`] structs
    students: Vec<Student>,
    /// Remaining "spots" for each [`Sex`]
    ///
    /// Represents (Male, Female)
    remaining_sex: (i16, i16),
    /// Remaining "spots" for each [`Grade`]
    ///
    /// Represents (Freshman, Sophomore, Junior, Senior)
    remaining_grade: (i16, i16, i16, i16),
}

impl std::fmt::Display for Advisory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&String> = self.advisors.iter().map(|t| &t.name).collect();
        write!(f, "(")?;
        match names.split_last() {
            Some(n) => {
                for i in n.1 {
                    write!(f, "{}, ", i)?
                }
            }
            None => {}
        };
        match names.split_last() {
            Some(n) => write!(f, "{})", n.0),
            None => write!(f, ")"),
        }
    }
}

impl Advisory {
    /// Adds a [`Student`] struct to the students vector
    pub(crate) fn add_student(&mut self, s: Student) {
        log::info!("Adding student {} to advisory {}", s, self);
        // Reduce number of remaining "spots" for the added student's sex
        if let Some(sex) = &s.sex {
            match sex {
                Sex::Male => self.remaining_sex.0 -= 1,
                Sex::Female => self.remaining_sex.1 -= 1,
            }
        }
        log::info!("Sex 'spots' in {}: {:?}", self, self.remaining_sex);
        // Reduce number of remaining "spots" for the added student's grade
        match s.grade {
            Grade::Freshman => self.remaining_grade.0 -= 1,
            Grade::Sophomore => self.remaining_grade.1 -= 1,
            Grade::Junior => self.remaining_grade.2 -= 1,
            Grade::Senior => self.remaining_grade.3 -= 1,
        }
        log::info!("Grade 'spots' in {}: {:?}", self, self.remaining_grade);

        self.students.push(s);
    }

    /// Gets the remaining number or "spots" left for a given sex in an advisory
    pub(crate) fn get_remaining_sex(&self, sex: &Option<Sex>) -> i16 {
        log::info!("Getting remaining 'spots' by sex");
        if let Some(sex) = sex {
            log::info!("Getting remaining 'spots' for {} in {}", sex, self);
            let num = match sex {
                Sex::Male => self.remaining_sex.0,
                Sex::Female => self.remaining_sex.1,
            };
            log::info!("{} has {} 'spots' left in {}", sex, num, self);
            num
        } else {
            log::info!("Sex inputted was None type");
            0
        }
    }

    /// Gets the remaining number of "spots" left for a given grade in an advisory
    pub(crate) fn get_remaining_grade(&self, grade: &Grade) -> i16 {
        log::info!("Getting remaining 'spots' for {} in {}", grade, self);
        let num = match grade {
            Grade::Freshman => self.remaining_grade.0,
            Grade::Sophomore => self.remaining_grade.1,
            Grade::Junior => self.remaining_grade.2,
            Grade::Senior => self.remaining_grade.3,
        };
        log::info!("{} has {} 'spots' left in {}", grade, num, self);
        num
    }

    /// Adds a [`Teacher`] struct to the advisors vector if Some
    pub(crate) fn add_teacher(&mut self, t: Option<Teacher>) {
        if let Some(t) = t {
            log::info!("Adding teacher {} to advisory {}", t, self);
            self.advisors.push(t);
        } else {
            log::info!("Added teacher is None type: doing nothing");
        }
    }

    /// Checks whether one of the advisors teaches the given student
    pub(crate) fn has_teacher(&self, s: &Student) -> bool {
        log::info!("Checking if {} has a teacher in {}", s, self);
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        log::info!("{} has a teacher in {}: {}", s, self, has);
        has
    }

    /// Default advisory values given target number of students for the advisory
    pub(crate) fn default(n: i16) -> Advisory {
        log::info!("Initialized new advisory via default");
        Self {
            advisors: Vec::<Teacher>::new(),
            students: Vec::<Student>::new(),
            // Set number of "spots" based on number of students in advisory
            remaining_sex: (n / 2, n / 2),
            remaining_grade: (n / 4, n / 4, n / 4, n / 4),
        }
    }
}

/// Weights from 0-10 used to assign importance to each possible parameter in the 'score calculation'
/// Used by [`AdvisoryForm`]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Weights {
    /// The importance that each student an an advisory has one of the advisors as a teacher
    ///
    /// Value from 0-10
    pub(crate) has_teacher: i8,
    /// The importance of biological sex diversity within advisories
    ///
    /// Value from 0-10
    pub(crate) sex_diverse: i8,
    /// The importance of grade diversity within advisories
    ///
    /// Value from 0-10
    pub(crate) grade_diverse: i8,
}

impl crate::Verify for Weights {
    fn verify(&self) -> bool {
        let range = 0..=10;
        range.contains(&self.has_teacher)
            && range.contains(&self.sex_diverse)
            && range.contains(&self.grade_diverse)
    }
}

/// Form for [`get_advisories`]'s input
#[derive(Serialize, Deserialize, Debug)]
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
