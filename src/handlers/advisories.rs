use crate::SharedState;
use axum::{extract::Extension, Json};
use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct Advisory {
    advisors: Vec<&'static str>,
    students: Vec<&'static str>,
}

pub async fn get_advisories(Extension(_state): Extension<Arc<SharedState>>) -> Json<Vec<Advisory>> {
    tracing::debug!("GET made to get_advisories");
    Json(build_advisories().await)
}

pub async fn build_advisories() -> Vec<Advisory> {
    tracing::debug!("Building advisories");
    vec![Advisory {
        advisors: vec!["John Smith"],
        students: vec!["Grant Lemons"],
    }]
}
