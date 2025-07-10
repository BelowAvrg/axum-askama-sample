use crate::error::AppError;
use sqlx::{migrate, postgres::PgPoolOptions, PgPool};
use std::env;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, AppError> {
        let todos: Vec<Todo> = sqlx::query_as!(
            Todo,
            "SELECT id, description, is_done FROM todos ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(todos)
    }

    pub async fn add_todo(&self, description: String) -> Result<(), AppError> {
        sqlx::query!("INSERT INTO todos (description) VALUES ($1)", description)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn toggle_todo(&self, id: i32) -> Result<(), AppError> {
        sqlx::query!("UPDATE todos SET is_done = NOT is_done WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM todos WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn rename_todo(&self, id: i32, description: String) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE todos SET description = $1 WHERE id = $2",
            description,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub is_done: bool,
}
