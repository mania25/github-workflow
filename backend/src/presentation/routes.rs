use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};

use super::handlers::{AppState, get_todos, get_todo, create_todo, update_todo, delete_todo};
use super::crypto_handlers::{CryptoState, exchange_keys};

pub fn create_router(state: Arc<AppState>) -> Router {
    let crypto_state = Arc::new(CryptoState {
        crypto: state.crypto.clone(),
    });

    Router::new()
        .route("/api/todos", get(get_todos))
        .route("/api/todos", post(create_todo))
        .route("/api/todos/:id", get(get_todo))
        .route("/api/todos/:id", put(update_todo))
        .route("/api/todos/:id", delete(delete_todo))
        .route("/api/crypto/exchange", post(exchange_keys))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
        .route("/api/crypto/exchange", post(exchange_keys).with_state(crypto_state))
}