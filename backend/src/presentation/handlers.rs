use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Json as RequestJson,
};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::TodoService;
use crate::domain::{CreateTodoRequest, UpdateTodoRequest};
use crate::infrastructure::PQCrypto;

pub struct AppState {
    pub todo_service: Arc<TodoService>,
    pub crypto: Arc<PQCrypto>,
}

pub async fn get_todos(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    match state.todo_service.get_all_todos().await {
        Ok(todos) => Ok(Json(json!(todos))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    match state.todo_service.get_todo_by_id(id).await {
        Ok(Some(todo)) => Ok(Json(json!(todo))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    RequestJson(request): RequestJson<CreateTodoRequest>,
) -> Result<Json<Value>, StatusCode> {
    match state.todo_service.create_todo(request).await {
        Ok(todo) => Ok(Json(json!(todo))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    RequestJson(request): RequestJson<UpdateTodoRequest>,
) -> Result<Json<Value>, StatusCode> {
    match state.todo_service.update_todo(id, request).await {
        Ok(Some(todo)) => Ok(Json(json!(todo))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    match state.todo_service.delete_todo(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
