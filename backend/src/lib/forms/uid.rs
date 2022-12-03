use serde::{Deserialize, Serialize};

/// Form for [`crate::database::clear_people`]'s input
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct UIDForm {
    /// The ID of the user's account within the database.
    ///
    /// Can be based on different things, like auth cred
    pub(crate) uid: String,
}
