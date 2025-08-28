use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, request: UpdateTodoRequest) {
        if let Some(title) = request.title {
            self.title = title;
        }
        if let Some(description) = request.description {
            self.description = Some(description);
        }
        if let Some(completed) = request.completed {
            self.completed = completed;
        }
        self.updated_at = Utc::now();
    }
}