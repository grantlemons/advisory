use serde::{Deserialize, Serialize};

/// Weights from 0-10 used to assign importance to each possible parameter in the 'score calculation'
/// Used by [`crate::forms::AdvisoryForm`]
#[derive(Deserialize, Serialize, Debug)]
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
