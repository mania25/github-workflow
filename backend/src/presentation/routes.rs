use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use super::crypto_handlers::exchange_keys;
use super::handlers::{create_todo, delete_todo, get_todo, get_todos, update_todo, AppState};

pub fn create_router(state: Arc<AppState>) -> Router {
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
}
