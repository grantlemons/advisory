use crate::{advisories::Weights, people::Teacher};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Form for [`crate::advisories::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub weights: Weights,
    /// Number of advisories to be generated
    pub num_advisories: u16,
    /// Groupings of teachers for advisories
    pub teacher_groupings: Arc<[Arc<[Teacher]>]>,
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
    /// #     teacher_groupings: vec![vec![Teacher::default(); 2]],
    /// # };
    /// settings.verify()?;
    /// # Ok(())
    /// # }
    /// ```
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        if self.num_advisories != self.teacher_groupings.len() as u16 {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            self.weights.verify()?;
            Ok(())
        }
    }
}
