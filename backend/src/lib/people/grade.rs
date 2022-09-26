use serde::{Deserialize, Serialize};

/// Representaion of possible grades for students
///
/// Adding more options requires changing the grade "spots" tuple in [`super::advisories::Advisory`] as well as adding the mapping to the implementations
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Grade {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

/// Mapping for string to [`Grade`] enum used for parsing info from database
impl From<i64> for Grade {
    fn from(n: i64) -> Self {
        match n {
            9 => Self::Freshman,
            10 => Self::Sophomore,
            11 => Self::Junior,
            12 => Self::Senior,
            _ => panic!("Grade must be from 9-12"),
        }
    }
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Freshman => "Freshman",
            Self::Sophomore => "Sophomore",
            Self::Junior => "Junior",
            Self::Senior => "Senior",
        };
        write!(f, "{}", string)
    }
}