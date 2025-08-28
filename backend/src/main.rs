mod domain;
mod application;
mod infrastructure;
mod presentation;

use std::sync::Arc;
use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;
use tracing_subscriber;

use application::TodoService;
use infrastructure::{SqliteTodoRepository, PQCrypto};
use presentation::{create_router, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:todos.db".to_string());

    let pool = Arc::new(SqlitePool::connect(&database_url).await?);
    
    let repository = Arc::new(SqliteTodoRepository::new(pool.clone()));
    repository.migrate().await?;
    
    let todo_service = Arc::new(TodoService::new(repository));
    let crypto = Arc::new(PQCrypto::new()?);

    let app_state = Arc::new(AppState {
        todo_service,
        crypto,
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;

    Ok(())
}
