use crate::SharedState;
use axum::{extract::Extension, http::StatusCode, Form, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// The role a person can have
/// Used in [`Person`] struct
#[derive(Serialize, Deserialize, Debug)]
enum PersonRole {
    Student,
    Advisor,
}

impl std::fmt::Display for PersonRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PersonRole::Student => write!(f, "student"),
            PersonRole::Advisor => write!(f, "advisor"),
        }
    }
}

/// Person form needed for [`add_person`]
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    role: PersonRole,
}

/// Handler to add a single person, either a advisor or a student to the database
/// Uses [`Person`] as a form for input
#[axum_macros::debug_handler]
//TODO: actually add node to remote database
pub async fn add_person(
    Form(person): Form<Person>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Person>, StatusCode> {
    tracing::debug!("POST made to people");
    tracing::debug!("New person {:?} added", person);
    Ok(Json(person))
}
