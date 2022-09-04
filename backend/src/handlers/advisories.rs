use super::people::*;
use crate::SharedState;
use axum::{extract::Extension, Json};
use neo4rs::*;
use std::sync::Arc;

/// Representation of an advisory
#[derive(serde::Serialize, Clone, Debug)]
pub struct Advisory {
    /// Vector of [`Teacher`] structs
    advisors: Vec<Teacher>,
    /// Vector of [`Student`] structs
    students: Vec<Student>,
    /// Remaining "spots" for each [`Sex`]
    /// Represents (Male, Female)
    remaining_sex: (i16, i16),
    /// Remaining "spots" for each [`Grade`]
    /// Represents (Freshman, Sophomore, Junior, Senior)
    remaining_grade: (i16, i16, i16, i16),
}

impl Advisory {
    /// Adds a [`Student`] struct to the students vector
    pub fn add_student(&mut self, s: Student) {
        // Reduce number of remaining "spots" for the added student's sex
        if let Some(sex) = &s.sex {
            match sex {
                Sex::Male => self.remaining_sex.0 -= 1,
                Sex::Female => self.remaining_sex.1 -= 1,
            }
        }
        // Reduce number of remaining "spots" for the added student's grade
        match s.grade {
            Grade::Freshman => self.remaining_grade.0 -= 1,
            Grade::Sophomore => self.remaining_grade.1 -= 1,
            Grade::Junior => self.remaining_grade.2 -= 1,
            Grade::Senior => self.remaining_grade.3 -= 1,
        }
        self.students.push(s);
    }

    /// Gets the remaining number or "spots" left for a given sex in an advisory
    pub fn get_remaining_sex(&self, sex: &Option<Sex>) -> i16 {
        if let Some(sex) = sex {
            match sex {
                Sex::Male => self.remaining_sex.0,
                Sex::Female => self.remaining_sex.1,
            }
        } else {
            0
        }
    }

    /// Gets the remaining number of "spots" left for a given grade in an advisory
    pub fn get_remaining_grade(&self, grade: &Grade) -> i16 {
        match grade {
            Grade::Freshman => self.remaining_grade.0,
            Grade::Sophomore => self.remaining_grade.1,
            Grade::Junior => self.remaining_grade.2,
            Grade::Senior => self.remaining_grade.3,
        }
    }

    /// Adds a [`Teacher`] struct to the advisors vector
    pub fn add_teacher(&mut self, t: Teacher) {
        self.advisors.push(t);
    }

    /// Checks whether one of the advisors teaches the given student
    pub fn has_teacher(&self, s: &Student) -> bool {
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        has
    }

    /// Default advisory values given target number of students for the advisory
    pub fn default(n: i16) -> Advisory {
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
pub async fn get_advisories(state: Extension<Arc<SharedState>>) -> Json<Vec<Advisory>> {
    tracing::debug!("GET made to get_advisories");
    Json(build_advisories(state).await)
}

/// Places students into advisories and returns a vector of them
/// Called by [`get_advisories`]
pub async fn build_advisories(Extension(state): Extension<Arc<SharedState>>) -> Vec<Advisory> {
    tracing::debug!("Building advisories");
    // create vectors from data from database
    let students: Vec<Student> = get_students(&state).await;
    let mut teachers = get_teachers(&state).await;

    // create vector of advisories to fill
    let s: i16 = students.len() as i16;
    let a: i16 = state.num_advisories;
    let mut advisories: Vec<Advisory> = vec![Advisory::default(s / a); a.try_into().unwrap()];

    // add teachers to advisories
    for i in &mut advisories {
        i.add_teacher(teachers.remove(0));
        i.add_teacher(teachers.remove(0));
        println!("{:?}", i.advisors);
    }
    // add students to advisories
    for i in students {
        let max: Option<usize> = advisories
            .iter()
            .map(|x| {
                (state.weights.has_teacher as i32 * x.has_teacher(&i) as i32 * (s / a) as i32)
                    + (state.weights.sex_diverse as i32 * x.get_remaining_sex(&i.sex) as i32)
                    + (state.weights.grade_diverse as i32 * x.get_remaining_grade(&i.grade) as i32)
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);
        if let Some(max) = max {
            println!("Adding {} to {:?}", &i.name, &advisories[max].advisors);
            advisories[max].add_student(i);
        }
    }
    advisories
}

/// Helper function for [`build_advisories`] to get vector of students from neo4j database using [`neo4rs`]
async fn get_students(state: &Arc<SharedState>) -> Vec<Student> {
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

        // Get the student's teachers
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
                println!("Teachers is empty ({})", name)
            }
        }

        // Add student with all fields to the students vector
        students.push(Student {
            name,
            teachers: t_structs,
            grade,
            sex,
        })
    }
    students
}

/// Helper function for [`build_advisories`] to get vector of teachers from neo4j database using [`neo4rs`]
async fn get_teachers(state: &Arc<SharedState>) -> Vec<Teacher> {
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

        // Add teacher will all fields to the teachers vector
        teachers.push(Teacher { name, sex })
    }
    teachers
}
