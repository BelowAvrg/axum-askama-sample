use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use askama::Template;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use axum_askama_sample::{
    database::{Database, Todo},
    error::AppError,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    todos: Vec<Todo>,
}

#[derive(serde::Deserialize)]
struct NewTodo {
    description: String,
}

#[derive(serde::Deserialize)]
struct RenameTodo {
    description: String,
}

async fn index(State(database): State<Database>) -> Result<impl IntoResponse, AppError> {
    let todos = database.get_todos().await?;
    Ok(IndexTemplate { todos })
}

async fn add_todo(State(database): State<Database>, Form(new_todo): Form<NewTodo>) -> Result<impl IntoResponse, AppError> {
    database.add_todo(new_todo.description).await?;
    Ok(Redirect::to("/"))
}

async fn toggle_todo(State(database): State<Database>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    database.toggle_todo(id).await?;
    Ok(Redirect::to("/"))
}

async fn delete_todo(State(database): State<Database>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    database.delete_todo(id).await?;
    Ok(Redirect::to("/"))
}

async fn rename_todo(State(database): State<Database>, Path(id): Path<i32>, Form(todo): Form<RenameTodo>) -> Result<impl IntoResponse, AppError> {
    database.rename_todo(id, todo.description).await?;
    Ok(Redirect::to("/"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_askama_sample=debug".into()),
        ))
        .init();

    let database = Database::new().await?;

    let app = Router::new()
        .route("/", get(index).post(add_todo))
        .route("/toggle/:id", post(toggle_todo))
        .route("/delete/:id", post(delete_todo))
        .route("/rename/:id", post(rename_todo))
        .with_state(database);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}