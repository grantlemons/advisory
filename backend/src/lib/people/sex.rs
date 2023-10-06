use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Representation of possible sexes for students within database
///
/// Adding more options requires changing the sex "spots" tuple in [`crate::advisories::Advisory`] as well as adding the mapping to the implementations.
///
/// I understand that grouping it like this might be somewhat sensitive, but it is needed for attempting diversity in the advisories. Sex is used in place of gender to avoid
/// complexities and ambiguity by representing biological sex. I know that there are some exceptions, but there is no pressing need to accommodate that edge case currently.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Sex {
    /// Represented in database as string value `"Male"`
    Male,
    /// Represented in database as string value `"Female"`
    Female,
}

/// Mapping for string to [`Sex`] enum used for parsing info from database
impl From<String> for Sex {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Male" => Self::Male,
            "Female" => Self::Female,
            other_value => panic!("{} not in list of sexes", other_value),
        }
    }
}

impl Into<Arc<str>> for Sex {
    fn into(self) -> Arc<str> {
        let str = match self {
            Self::Male => "Male",
            Self::Female => "Female",
        };

        Arc::from(str)
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Male => "Male",
            Self::Female => "Female",
        };
        write!(f, "{}", string)
    }
}
