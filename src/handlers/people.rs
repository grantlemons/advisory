use crate::SharedState;
use axum::{extract::Extension, http::StatusCode, Form, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
/// The role a person can have
/// Used in [`Person`] struct
enum PersonRole {
    Student,
    Teacher,
}

#[derive(Serialize, Deserialize, Debug)]
/// Person form needed for [`add_person`]
pub struct Person {
    name: String,
    role: PersonRole,
}

//TODO: actually add node to remote database
#[axum_macros::debug_handler]
/// Handler to add a single person, either a teacher or a student to the database
/// Uses [`Person`] as a form for input
pub async fn add_person(
    Form(person): Form<Person>,
    Extension(_state): Extension<Arc<SharedState>>,
) -> Result<Json<Person>, StatusCode> {
    tracing::debug!("POST made to people");
    tracing::debug!("New person {:?} added", person);
    Ok(Json(person))
}
