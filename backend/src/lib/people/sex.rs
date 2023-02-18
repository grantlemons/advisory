use serde::{Deserialize, Serialize};

/// Representation of possible sexes for students within database
///
/// Adding more options requires changing the sex "spots" tuple in [`crate::advisories::Advisory`] as well as adding the mapping to the implementations.
///
/// I understand that grouping it like this might be somewhat sensitive, but it is needed for attempting diversity in the advisories. Sex is used in place of gender to avoid
/// complexities and ambiguity by representing biological sex. I know that there are some exceptions, but there is no pressing need to accommodate that edge case currently.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Sex {
    Male,
    Female,
}

/// Mapping for string to [`Sex`] enum used for parsing info from database
impl<T: Into<String>> From<T> for Sex {
    fn from(s: T) -> Self {
        match s.into().as_str() {
            "Male" => Self::Male,
            "Female" => Self::Female,
            other_value => panic!("{} not in list of sexes", other_value),
        }
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
