use serde::{Deserialize, Serialize};

/// Weights from 1-10 used to assign importance to each possible parameter in the 'score calculation'
/// Used by [`crate::advisories::Advisory`]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Weights {
    /// The relative importance that each student an an advisory has one of the advisors as a teacher
    ///
    /// Value from 1-10
    pub has_teacher: i8,
    /// The relative importance of biological sex diversity within advisories
    ///
    /// Value from 1-10
    pub sex_diverse: i8,
    /// The relative importance of grade diversity within advisories
    ///
    /// Value from 1-10
    pub grade_diverse: i8,
    /// The relative importance of the advisories having an equal amount of people in them
    ///
    /// Value from 1-10
    pub equal_people: i8,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            has_teacher: 1,
            sex_diverse: 1,
            grade_diverse: 1,
            equal_people: 1,
        }
    }
}

impl crate::Verify for Weights {
    /// Returns an [`axum::http::StatusCode`] type, so errors can be passed through to handlers
    ///
    /// # Examples
    ///
    /// ```
    /// # use advisory_backend_lib::{Verify, advisories::Weights};
    /// fn func() -> Result<(), axum::http::StatusCode> {
    ///     let weights = Weights {
    ///         has_teacher: 8,
    ///         sex_diverse: 9,
    ///         grade_diverse: 10,
    ///         equal_people: 10,
    ///     };
    ///     weights.verify()?;
    ///     Ok(())
    /// }
    /// assert_eq!(func(), Ok(()))
    /// ```
    ///
    /// ```
    /// # use advisory_backend_lib::{Verify, advisories::Weights};
    /// fn func() -> Result<(), axum::http::StatusCode> {
    ///     let weights = Weights {
    ///         has_teacher: 8,
    ///         sex_diverse: 11,
    ///         grade_diverse: 10,
    ///         equal_people: 10,
    ///     };
    ///     weights.verify()?;
    ///     Ok(())
    /// }
    /// assert_ne!(func(), Ok(()))
    /// ```
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        let range = 1..=10;
        if !(range.contains(&self.has_teacher)
            && range.contains(&self.sex_diverse)
            && range.contains(&self.grade_diverse)
            && range.contains(&self.equal_people))
        {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            Ok(())
        }
    }
}
