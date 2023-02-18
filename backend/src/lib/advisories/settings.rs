use crate::{advisories::Weights, people::Teacher};
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Settings {
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub(crate) weights: Weights,
    /// Number of advisories to be generated
    pub(crate) num_advisories: i16,
    /// Pairs of teachers for advisories
    pub(crate) teacher_pairs: Vec<[Option<Teacher>; 2]>,
}

impl crate::Verify for Settings {
    fn verify(&self) -> bool {
        self.weights.verify() && self.num_advisories > 0
    }
}
