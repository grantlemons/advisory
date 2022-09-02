use super::people::*;
use crate::SharedState;
use axum::{extract::Extension, Json};
use std::sync::Arc;

#[derive(serde::Serialize, Clone)]
pub struct Advisory {
    advisors: Vec<Teacher>,
    students: Vec<Student>,
    remaining_sex: (i16, i16),
    remaining_grade: (i16, i16, i16, i16),
}

impl Advisory {
    pub fn add_student(mut self, s: Student) {
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

    pub fn add_teacher(mut self, t: Teacher) {
        self.advisors.push(t);
    }

    pub fn has_teacher(&self, s: &Student) -> bool {
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        println!("{} has a teacher!", s.name);
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

pub async fn get_advisories(state: Extension<Arc<SharedState>>) -> Json<Vec<Advisory>> {
    tracing::debug!("GET made to get_advisories");
    Json(build_advisories(state).await)
}

pub async fn build_advisories(Extension(state): Extension<Arc<SharedState>>) -> Vec<Advisory> {
    tracing::debug!("Building advisories");
    let a = state.num_advisories;
    let s = state.num_students;
    let weights = &state.weights;
    let advisories: Vec<Advisory> = vec![Advisory::default(a); a.try_into().unwrap()];
    let students: Vec<Student> = vec![Student::default(); s.try_into().unwrap()];

    for i in students {
        let max: Option<usize> = advisories
            .iter()
            .map(|x| {
                (weights.has_teacher as i32 * x.has_teacher(&i) as i8 as i32)
                    + (weights.sex_diverse as i32 * x.get_remaining_sex(&i.sex) as i32)
                    + (weights.grade_diverse as i32 * x.get_remaining_grade(&i.grade) as i32)
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);
        if let Some(max) = max {
            // advisories[max].add_student(Student::default());
        }
    }
    advisories
}
