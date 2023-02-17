use crate::{people::Teacher, Weights};
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

impl Settings {
    pub fn populate_advisories(&self, advisories: &mut [crate::advisories::Advisory]) {
        for (index, target_advisory) in advisories.iter_mut().enumerate() {
            let [t1, t2] = self.teacher_pairs[index].clone();

            log::info!("Adding {:?} to {}", vec![&t1, &t2], target_advisory);
            target_advisory.add_teacher(t1);
            target_advisory.add_teacher(t2);
        }
    }
}
