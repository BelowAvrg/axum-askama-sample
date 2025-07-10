use crate::{
    database::{Database, Todo},
    error::AppError,
};
use askama::Template;
use axum::{debug_handler, response::Html};
use axum::{
    extract::{Path, State, FromRequest, Request},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use validator::Validate;

pub fn create_router(database: Database) -> Router {
    Router::new()
        .route("/", get(index).post(add_todo))
        .route("/toggle/{id}", post(toggle_todo))
        .route("/delete/{id}", post(delete_todo))
        .route("/rename/{id}", post(rename_todo))
        .with_state(database)
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<Todo>,
}

#[debug_handler]
pub async fn index(State(database): State<Database>) -> Result<impl IntoResponse, AppError> {
    let todos = database.get_todos().await?;
    Ok(Html(IndexTemplate { todos }.render()?))
}

#[derive(serde::Deserialize, Validate)]
pub struct NewTodo {
    #[validate(length(min = 1, max = 25))]
    pub description: String,
}

#[debug_handler]
pub async fn add_todo(
    State(database): State<Database>,
    ValidatedForm(new_todo): ValidatedForm<NewTodo>,
) -> Result<impl IntoResponse, AppError> {
    database.add_todo(new_todo.description).await?;
    Ok(Redirect::to("/"))
}

#[derive(serde::Deserialize, Validate)]
pub struct RenameTodo {
    #[validate(length(min = 1, max = 25))]
    pub description: String,
}

#[debug_handler]
pub async fn rename_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
    ValidatedForm(todo): ValidatedForm<RenameTodo>,
) -> Result<impl IntoResponse, AppError> {
    database.rename_todo(id, todo.description).await?;
    Ok(Redirect::to("/"))
}

#[debug_handler]
pub async fn toggle_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    database.toggle_todo(id).await?;
    Ok(Redirect::to("/"))
}

#[debug_handler]
pub async fn delete_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    database.delete_todo(id).await?;
    Ok(Redirect::to("/"))
}

pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data) = Form::<T>::from_request(req, state).await?;
        data.validate()?;
        Ok(ValidatedForm(data))
    }
}

