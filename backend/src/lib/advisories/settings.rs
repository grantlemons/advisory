use crate::{advisories::Weights, people::Teacher};
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub weights: Weights,
    /// Number of advisories to be generated
    pub num_advisories: i16,
    /// Pairs of teachers for advisories
    pub teacher_pairs: Vec<[Option<Teacher>; 2]>,
}

impl crate::Verify for Settings {
    /// Returns an [`axum::http::StatusCode`] type, so errors can be passed through to handlers
    ///
    /// # Example
    ///
    /// ```
    /// # use advisory_backend_lib::{Verify, advisories::{Settings, Weights}, people::Teacher};
    /// # fn main() -> Result<(), axum::http::StatusCode> {
    /// # let settings = Settings {
    /// #     weights: Weights::default(),
    /// #     num_advisories: 1,
    /// #     teacher_pairs: vec![[Some(Teacher::default()), Some(Teacher::default())]],
    /// # };
    /// settings.verify()?;
    /// # Ok(())
    /// # }
    /// ```
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        if self.num_advisories != self.teacher_pairs.len() as i16 {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            self.weights.verify()?;
            Ok(())
        }
    }
}
