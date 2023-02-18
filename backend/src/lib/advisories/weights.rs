use serde::{Deserialize, Serialize};

/// Weights from 0-10 used to assign importance to each possible parameter in the 'score calculation'
/// Used by [`crate::advisories::Advisory`]
#[derive(Deserialize, Serialize, Debug)]
pub struct Weights {
    /// The importance that each student an an advisory has one of the advisors as a teacher
    ///
    /// Value from 0-10
    pub has_teacher: i8,
    /// The importance of biological sex diversity within advisories
    ///
    /// Value from 0-10
    pub sex_diverse: i8,
    /// The importance of grade diversity within advisories
    ///
    /// Value from 0-10
    pub grade_diverse: i8,
}

impl crate::Verify for Weights {
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        let range = 0..=10;
        if !range.contains(&self.has_teacher)
            && range.contains(&self.sex_diverse)
            && range.contains(&self.grade_diverse)
        {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            Ok(())
        }
    }
}
