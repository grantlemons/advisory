use crate::SharedState;
use anyhow::{anyhow, Result};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub(crate) struct UserData {
    auth_time: u64,
    client_id: String,
    #[serde(rename = "cognito:groups", default)]
    groups: std::collections::HashSet<String>,
    event_id: String,
    exp: u64,
    iat: u64,
    iss: String,
    jti: String,
    origin_jti: String,
    scope: String,
    sub: String,
    token_use: String,
    username: String,
}

impl UserData {
    pub(crate) fn user_id(&self) -> &str {
        &self.sub
    }

    pub(crate) fn is_member(&self, group: &str) -> bool {
        self.groups.contains(group)
    }
}

/// Tower layer that adds an [`Option<UserData>`] as an extension to the request
/// This can be used by handlers for authentication as well as for the value
pub(crate) async fn auth<B>(
    State(state): State<SharedState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let mut user: Option<UserData> = None;

    if let Some(a_header) = auth_header {
        user = verify_jwt(a_header, state).await;
    }
    req.extensions_mut().insert(user);
    next.run(req).await
}

/// Interface for [`decrypt_jwt`]
pub(crate) async fn verify_jwt(token: &str, state: SharedState) -> Option<UserData> {
    match decrypt_jwt(token, &state.keyset, &state.verifier).await {
        Ok(user) => Some(user),
        Err(_) => {
            log::info!("Failed JWT Verification");
            None
        }
    }
}

/// Decrypt JWT token from Cognito
async fn decrypt_jwt(
    token: &str,
    keyset: &jsonwebtokens_cognito::KeySet,
    verifier: &jsonwebtokens::Verifier,
) -> Result<UserData> {
    let json = match keyset.verify(token, verifier).await {
        Ok(value) => Ok(value),
        Err(err) => Err(anyhow!("{}", err)),
    }?;
    Ok(serde_json::from_value(json)?)
}
