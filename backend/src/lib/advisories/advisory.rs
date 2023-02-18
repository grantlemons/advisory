use crate::{
    advisories::Weights,
    people::{Grade, Sex, Student, Teacher},
};
use serde::{Deserialize, Serialize};

/// Representation of an advisory
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Advisory {
    /// Vector of [`Teacher`] structs
    advisors: Vec<Teacher>,
    /// Vector of [`Student`] structs
    students: Vec<Student>,
    /// Remaining quota for each [`Sex`]
    ///
    /// Represents (Male, Female)
    remaining_sex: [i16; 2],
    /// Remaining quota for each [`Grade`]
    ///
    /// Represents (Freshman, Sophomore, Junior, Senior)
    remaining_grade: [i16; 4],
}

impl std::fmt::Display for Advisory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&String> = self.advisors.iter().map(|t| &t.name).collect();
        write!(f, "(")?;
        if let Some(n) = names.split_last() {
            for i in n.1 {
                write!(f, "{}, ", i)?
            }
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
        // Reduce number of remaining "spots" for the added student's sex
        match s.sex {
            Some(Sex::Male) => self.remaining_sex[0] -= 1,
            Some(Sex::Female) => self.remaining_sex[1] -= 1,
            None => {}
        }
        // Reduce number of remaining "spots" for the added student's grade
        match s.grade {
            Grade::Freshman => self.remaining_grade[0] -= 1,
            Grade::Sophomore => self.remaining_grade[1] -= 1,
            Grade::Junior => self.remaining_grade[2] -= 1,
            Grade::Senior => self.remaining_grade[3] -= 1,
        }

        self.students.push(s);
    }

    /// Gets the remaining number or "spots" left for a given sex in an advisory
    pub(crate) fn get_remaining_sex(&self, sex: &Option<Sex>) -> i16 {
        match sex {
            Some(Sex::Male) => self.remaining_sex[0],
            Some(Sex::Female) => self.remaining_sex[1],
            None => 0,
        }
    }

    /// Gets the remaining number of "spots" left for a given grade in an advisory
    pub(crate) fn get_remaining_grade(&self, grade: &Grade) -> i16 {
        match grade {
            Grade::Freshman => self.remaining_grade[0],
            Grade::Sophomore => self.remaining_grade[1],
            Grade::Junior => self.remaining_grade[2],
            Grade::Senior => self.remaining_grade[3],
        }
    }

    /// Adds a [`Teacher`] struct to the advisors vector if Some
    pub(crate) fn add_teacher(&mut self, t: Option<Teacher>) {
        if let Some(t) = t {
            self.advisors.push(t);
        } else {
            log::info!("Added teacher is None type: doing nothing");
        }
    }

    /// Checks whether one of the advisors teaches the given student
    pub(crate) fn has_teacher(&self, s: &Student) -> bool {
        let mut has = false;
        for i in &s.teachers {
            if self.advisors.contains(i) {
                has = true;
            }
        }
        has
    }

    /// Default advisory values given target number of students for the advisory
    pub(crate) fn new(n: i16) -> Advisory {
        log::info!("Initialized new advisory via new");
        Self {
            advisors: Vec::<Teacher>::new(),
            students: Vec::<Student>::new(),
            // Set number of "spots" based on number of students in advisory
            remaining_sex: [n / 2, n / 2],
            remaining_grade: [n / 4, n / 4, n / 4, n / 4],
        }
    }

    /// Calculate a weight between the advisory and a student
    /// This value compensates for what the user deems important with weights assigned to the different parameters
    pub(crate) fn calculate_weight(
        &self,
        student: &Student,
        weights: &Weights,
        students_per_advisory: i16,
    ) -> i32 {
        let number_of_sexes: i32 = self.remaining_sex.len() as i32;
        let number_of_grades: i32 = self.remaining_grade.len() as i32;

        let teacher_weighted_value = weights.has_teacher as i32
            * students_per_advisory as i32
            * self.has_teacher(student) as i32;
        let sexes_weighted_value = number_of_sexes
            * (weights.sex_diverse as i32 * self.get_remaining_sex(&student.sex) as i32);
        let grade_weighted_value = number_of_grades
            * (weights.grade_diverse as i32 * self.get_remaining_grade(&student.grade) as i32);
        teacher_weighted_value + sexes_weighted_value + grade_weighted_value
    }
}
