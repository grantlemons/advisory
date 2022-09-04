use super::people::*;
use crate::SharedState;
use axum::{extract::Extension, Json};
use neo4rs::*;
use std::sync::Arc;

/// Representation of an advisory
#[derive(serde::Serialize, Clone, Debug)]
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

impl Advisory {
    /// Adds a [`Student`] struct to the students vector
    pub(crate) fn add_student(&mut self, s: Student) {
        log::trace!("Adding student {:?} to advisory {:?}", s, self);
        // Reduce number of remaining "spots" for the added student's sex
        if let Some(sex) = &s.sex {
            match sex {
                Sex::Male => self.remaining_sex.0 -= 1,
                Sex::Female => self.remaining_sex.1 -= 1,
            }
        }
        log::trace!("Sex 'spots': {:?}", self.remaining_sex);
        // Reduce number of remaining "spots" for the added student's grade
        match s.grade {
            Grade::Freshman => self.remaining_grade.0 -= 1,
            Grade::Sophomore => self.remaining_grade.1 -= 1,
            Grade::Junior => self.remaining_grade.2 -= 1,
            Grade::Senior => self.remaining_grade.3 -= 1,
        }
        log::trace!("Grade 'spots': {:?}", self.remaining_grade);

        self.students.push(s);
    }

    /// Gets the remaining number or "spots" left for a given sex in an advisory
    pub(crate) fn get_remaining_sex(&self, sex: &Option<Sex>) -> i16 {
        log::trace!("Getting remaining 'spots' by sex");
        if let Some(sex) = sex {
            log::trace!("Getting remaining 'spots' for {:?}", sex);
            let num = match sex {
                Sex::Male => self.remaining_sex.0,
                Sex::Female => self.remaining_sex.1,
            };
            log::trace!("{:?} has {} 'spots' left", sex, num);
            num
        } else {
            log::trace!("Sex inputted was None type");
            0
        }
    }

    /// Gets the remaining number of "spots" left for a given grade in an advisory
    pub(crate) fn get_remaining_grade(&self, grade: &Grade) -> i16 {
        log::trace!("Getting remaining 'spots' for {:?}", grade);
        let num = match grade {
            Grade::Freshman => self.remaining_grade.0,
            Grade::Sophomore => self.remaining_grade.1,
            Grade::Junior => self.remaining_grade.2,
            Grade::Senior => self.remaining_grade.3,
        };
        log::trace!("{:?} has {} 'spots' left", grade, num);
        num
    }

    /// Adds a [`Teacher`] struct to the advisors vector if Some
    pub(crate) fn add_teacher(&mut self, t: Option<Teacher>) {
        if let Some(t) = t {
            log::trace!("Adding teacher {:?} to advisory {:?}", t, self);
            self.advisors.push(t);
        } else {
            log::trace!("Added teacher is None type: doing nothing");
        }
    }

    /// Checks whether one of the advisors teaches the given student
    pub(crate) fn has_teacher(&self, s: &Student) -> bool {
        log::trace!("Checking if {:?} has a teacher in {:?}", s, self);
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        log::trace!("{:?} has a teacher in {:?}: {}", s, self, has);
        has
    }

    /// Default advisory values given target number of students for the advisory
    pub(crate) fn default(n: i16) -> Advisory {
        log::trace!("Initialized new advisory via default");
        Self {
            advisors: Vec::<Teacher>::new(),
            students: Vec::<Student>::new(),
            // Set number of "spots" based on number of students in advisory
            remaining_sex: (n / 2, n / 2),
            remaining_grade: (n / 4, n / 4, n / 4, n / 4),
        }
    }
}

/// Wrapper of [`build_advisories`] called by https get requests to `/`
pub(crate) async fn get_advisories(state: Extension<Arc<SharedState>>) -> Json<Vec<Advisory>> {
    log::debug!("GET made to get_advisories");
    Json(build_advisories(state).await)
}

/// Places students into advisories and returns a vector of them
///
/// Called by [`get_advisories`]
pub(crate) async fn build_advisories(
    Extension(state): Extension<Arc<SharedState>>,
) -> Vec<Advisory> {
    log::debug!("Building advisories");
    // create vectors from data from database
    let students: Vec<Student> = get_students(&state).await;
    let mut teachers = get_teachers(&state).await;

    // create vector of advisories to fill
    let s: i16 = students.len() as i16;
    let a: i16 = state.num_advisories;
    log::trace!("{} Students, {} Advisories", s, a);
    let mut advisories: Vec<Advisory> = vec![Advisory::default(s / a); a.try_into().unwrap()];

    // add teachers to advisories
    for i in &mut advisories {
        let t1 = teachers.pop();
        let t2 = teachers.pop();
        log::trace!("Adding {:?} & {:?} to {:?}", t1, t2, i);
        i.add_teacher(t1);
        i.add_teacher(t2);
    }
    // add students to advisories
    for i in students {
        let max: Option<usize> = advisories
            .iter()
            .map(|x| {
                log::trace!("Calculating weight for {} & {:?}", i.name, x);
                let weight = (state.weights.has_teacher as i32
                    * x.has_teacher(&i) as i32
                    * (s / a) as i32)
                    + (state.weights.sex_diverse as i32 * x.get_remaining_sex(&i.sex) as i32)
                    + (state.weights.grade_diverse as i32 * x.get_remaining_grade(&i.grade) as i32);
                log::trace!("Weight for {} & {:?} is {}", i.name, x, weight);
                weight
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);
        if let Some(max) = max {
            log::trace!("Adding {} to {:?}", i.name, advisories[max].advisors);
            advisories[max].add_student(i);
        }
    }
    log::trace!(
        "build_advisories complete: advisories are {:#?}",
        advisories
    );
    advisories
}

/// Helper function for [`build_advisories`] to get vector of students from neo4j database using [`neo4rs`]
async fn get_students(state: &Arc<SharedState>) -> Vec<Student> {
    log::trace!("Getting students from database");

    // Get the result of a Cypher query to the neo4j database
    let mut result = state.graph
        .execute(query("MATCH (s:Student)<-[:TEACHES]-(t) RETURN distinct(s) as students, collect(t) as teachers"))
        .await
        .unwrap();

    // Create and initialize returned vector
    let mut students: Vec<Student> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        // Get student data from returned row of the database query
        let student: Node = row.get("students").unwrap();
        let name: String = student.get("name").unwrap();
        let grade: Grade = Grade::from(student.get::<i64>("grade").unwrap());
        let sex: Option<Sex> = Some(Sex::from(student.get::<String>("sex").unwrap()));

        log::trace!(
            "Student data is {{name: {}, grade: {:?}, sex: {:?}}}",
            name,
            grade,
            sex
        );

        // Get the student's teachers
        log::trace!("Getting {}'s teachers", name);
        let mut t_structs: Vec<Teacher> = Vec::new();
        match row.get::<Vec<Node>>("teachers") {
            Some(teachers) => {
                t_structs = teachers
                    .into_iter()
                    .map(|t| Teacher {
                        name: t.get("name").unwrap(),
                        sex: Some(Sex::from(t.get::<String>("sex").unwrap())),
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
        log::trace!("Adding student to vec: {:?}", student);
        students.push(student)
    }
    log::trace!("Done getting students!");
    students
}

/// Helper function for [`build_advisories`] to get vector of teachers from neo4j database using [`neo4rs`]
async fn get_teachers(state: &Arc<SharedState>) -> Vec<Teacher> {
    log::trace!("Getting teachers from database");

    // Get the result of a Cypher query to the neo4j database
    let mut result = state
        .graph
        .execute(query("MATCH (t:Teacher) RETURN distinct(t) as teachers"))
        .await
        .unwrap();

    // Create and initialize returned vector
    let mut teachers: Vec<Teacher> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        // Get teacher data from returned row of the database query
        let teacher: Node = row.get("teachers").unwrap();
        let name: String = teacher.get("name").unwrap();
        let sex: Option<Sex> = Some(Sex::from(teacher.get::<String>("sex").unwrap()));

        log::trace!("Teacher data is {{name: {}, sex: {:?}}}", name, sex);

        // Add teacher will all fields to the teachers vector
        let teacher = Teacher { name, sex };
        log::trace!("Adding teacher to vec: {:?}", teacher);
        teachers.push(teacher)
    }
    log::trace!("Done getting teachers!");
    teachers
}
