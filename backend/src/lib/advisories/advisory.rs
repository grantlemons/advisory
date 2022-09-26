use crate::people::{grade::Grade, sex::Sex, student::Student, teacher::Teacher};
use serde::{Deserialize, Serialize};

/// Representation of an advisory
#[derive(Deserialize, Serialize, Clone, Debug)]
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