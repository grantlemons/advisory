use crate::Weights;
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Settings {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) user_id: String,
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub(crate) weights: Weights,
    /// Number of advisories to be generated
    pub(crate) num_advisories: i16,
}

impl crate::Verify for Settings {
    fn verify(&self) -> bool {
        !self.user_id.is_empty() && self.weights.verify() && self.num_advisories > 0
    }
}
