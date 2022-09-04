use super::people::*;
use crate::SharedState;
use axum::{extract::Extension, Json};
use neo4rs::*;
use std::sync::Arc;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Advisory {
    advisors: Vec<Teacher>,
    students: Vec<Student>,
    remaining_sex: (i16, i16),
    remaining_grade: (i16, i16, i16, i16),
}

impl Advisory {
    pub fn add_student(&mut self, s: Student) {
        if let Some(sex) = &s.sex {
            match sex {
                Sex::Male => self.remaining_sex.0 -= 1,
                Sex::Female => self.remaining_sex.1 -= 1,
            }
        }
        match s.grade {
            Grade::Freshman => self.remaining_grade.0 -= 1,
            Grade::Sophomore => self.remaining_grade.1 -= 1,
            Grade::Junior => self.remaining_grade.2 -= 1,
            Grade::Senior => self.remaining_grade.3 -= 1,
        }
        self.students.push(s);
    }

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

    pub fn get_remaining_grade(&self, grade: &Grade) -> i16 {
        match grade {
            Grade::Freshman => self.remaining_grade.0,
            Grade::Sophomore => self.remaining_grade.1,
            Grade::Junior => self.remaining_grade.2,
            Grade::Senior => self.remaining_grade.3,
        }
    }

    pub fn add_teacher(&mut self, t: Teacher) {
        self.advisors.push(t);
    }

    pub fn has_teacher(&self, s: &Student) -> bool {
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        has
    }

    pub fn default(n: i16) -> Advisory {
        Self {
            advisors: Vec::<Teacher>::new(),
            students: Vec::<Student>::new(),
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

/// Helper function for [`build_advisories`] to get vector of students from neo4j database
async fn get_students(state: &Arc<SharedState>) -> Vec<Student> {
    let mut result = state.graph
        .execute(query("MATCH (s:Student)<-[:TEACHES]-(t) RETURN distinct(s) as students, collect(t) as teachers"))
        .await
        .unwrap();
    let mut students: Vec<Student> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        let student: Node = row.get("students").unwrap();
        let name: String = student.get("name").unwrap();
        let grade: Grade = Grade::from(student.get::<i64>("grade").unwrap());
        let sex: Option<Sex> = Some(Sex::from(student.get::<String>("sex").unwrap()));

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
        students.push(Student {
            name,
            teachers: t_structs,
            grade,
            sex,
        })
    }
    students
}

/// Helper function for [`build_advisories`] to get vector of teachers from neo4j database
async fn get_teachers(state: &Arc<SharedState>) -> Vec<Teacher> {
    let mut result = state
        .graph
        .execute(query("MATCH (t:Teacher) RETURN distinct(t) as teachers"))
        .await
        .unwrap();
    let mut teachers: Vec<Teacher> = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        let teacher: Node = row.get("teachers").unwrap();
        let name: String = teacher.get("name").unwrap();
        let sex: Option<Sex> = Some(Sex::from(teacher.get::<String>("sex").unwrap()));

        teachers.push(Teacher { name, sex })
    }
    teachers
}
