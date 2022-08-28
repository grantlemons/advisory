use axum::Extension;
use std::sync::Arc;
use crate::SharedState;

pub async fn get_advisories(_state: Extension<Arc<SharedState>>) {

}
