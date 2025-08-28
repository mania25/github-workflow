use axum::{extract::State, http::StatusCode, response::Json, Json as RequestJson};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::presentation::handlers::AppState;

#[derive(Deserialize)]
pub struct KeyExchangeRequest {
    pub public_key: Vec<u8>,
}

#[derive(Serialize)]
pub struct KeyExchangeResponse {
    pub server_public_key: Vec<u8>,
}

pub async fn exchange_keys(
    State(state): State<Arc<AppState>>,
    RequestJson(request): RequestJson<KeyExchangeRequest>,
) -> Result<Json<KeyExchangeResponse>, StatusCode> {
    let client_public_key = request.public_key;

    match state.crypto.generate_session_key(&client_public_key) {
        Ok(server_public_key) => Ok(Json(KeyExchangeResponse {
            server_public_key: server_public_key.to_vec(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
