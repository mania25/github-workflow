use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::TodoRepository;
use crate::domain::Todo;

#[derive(Clone)]
pub struct SqliteTodoRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteTodoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(self.pool.as_ref())
            .await?;
        Ok(())
    }
}

#[async_trait]
impl TodoRepository for SqliteTodoRepository {
    async fn find_all(&self) -> Result<Vec<Todo>> {
        let rows = sqlx::query(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos ORDER BY created_at DESC"
        )
        .fetch_all(self.pool.as_ref())
        .await?;

        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                completed: row.get("completed"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(todos)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>> {
        let row = sqlx::query(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await?;

        if let Some(row) = row {
            Ok(Some(Todo {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                completed: row.get("completed"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, todo: Todo) -> Result<Todo> {
        sqlx::query(
            r#"
            INSERT INTO todos (id, title, description, completed, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(todo.id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .execute(self.pool.as_ref())
        .await?;

        Ok(todo)
    }

    async fn update(&self, todo: Todo) -> Result<Todo> {
        sqlx::query(
            r#"
            UPDATE todos
            SET title = ?2, description = ?3, completed = ?4, updated_at = ?5
            WHERE id = ?1
            "#,
        )
        .bind(todo.id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.updated_at)
        .execute(self.pool.as_ref())
        .await?;

        Ok(todo)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}
