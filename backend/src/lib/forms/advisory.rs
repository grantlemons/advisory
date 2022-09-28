use crate::advisories::weights::Weights;
use serde::{Deserialize, Serialize};

/// Form for [`crate::advisories::advisory::Advisory`]'s input
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct AdvisoryForm {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) uid: String,
    /// The respective value of each factor in the calculation of advisory 'scores'
    pub(crate) weights: Weights,
    /// Number of advisories to be generated
    pub(crate) num_advisories: i16,
}

impl crate::Verify for AdvisoryForm {
    fn verify(&self) -> bool {
        !self.uid.is_empty() && self.weights.verify() && self.num_advisories > 0
    }
}
