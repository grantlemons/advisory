use crate::{advisories::Weights, people::Teacher};
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub weights: Weights,
    /// Number of advisories to be generated
    pub num_advisories: i16,
    /// Pairs of teachers for advisories
    pub teacher_pairs: Vec<[Option<Teacher>; 2]>,
}

impl crate::Verify for Settings {
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        if !self.num_advisories > 0 {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            self.weights.verify()?;
            Ok(())
        }
    }
}
