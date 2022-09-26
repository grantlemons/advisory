use serde::{Deserialize, Serialize};

/// Representaion of possible sexes for students within database
///
/// Adding more options requires changing the sex "spots" tuple in [`super::advisories::Advisory`] as well as adding the mapping to the implementations.
///
/// I understand that grouping it like this might be somewhat sensitive, but it is needed for attempting diversity in the advisories. Sex is used in place of gender to avoid
/// complexities and ambiguity by representing biological sex. I know that there are some exceptions, but there is no pressing need to accommodate that edge case currently.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Sex {
    Male,
    Female,
}

/// Mapping for string to [`Sex`] enum used for parsing info from database
impl From<String> for Sex {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Male" => Self::Male,
            "Female" => Self::Female,
            _ => panic!("{} not in list of sexes", s),
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
