use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;

use crate::domain::{Todo, CreateTodoRequest, UpdateTodoRequest};

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Todo>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>>;
    async fn create(&self, todo: Todo) -> Result<Todo>;
    async fn update(&self, todo: Todo) -> Result<Todo>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct TodoService {
    repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>> {
        self.repository.find_all().await
    }

    pub async fn get_todo_by_id(&self, id: Uuid) -> Result<Option<Todo>> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_todo(&self, request: CreateTodoRequest) -> Result<Todo> {
        let todo = Todo::new(request.title, request.description);
        self.repository.create(todo).await
    }

    pub async fn update_todo(&self, id: Uuid, request: UpdateTodoRequest) -> Result<Option<Todo>> {
        if let Some(mut todo) = self.repository.find_by_id(id).await? {
            todo.update(request);
            let updated_todo = self.repository.update(todo).await?;
            Ok(Some(updated_todo))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_todo(&self, id: Uuid) -> Result<bool> {
        if self.repository.find_by_id(id).await?.is_some() {
            self.repository.delete(id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}