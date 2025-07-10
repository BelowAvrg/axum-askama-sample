use crate::{
    database::{Database, Todo},
    error::AppError,
};
use askama::Template;
use axum::extract::rejection::FormRejection;
use axum::{
    async_trait,
    extract::{FromRequest, Path, State},
    http::Request,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub fn create_router(database: Database) -> Router {
    Router::new()
        .route("/", get(index).post(add_todo))
        .route("/toggle/:id", post(toggle_todo))
        .route("/delete/:id", post(delete_todo))
        .route("/rename/:id", post(rename_todo))
        .with_state(database)
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<Todo>,
}

pub async fn index(State(database): State<Database>) -> Result<impl IntoResponse, AppError> {
    let todos = database.get_todos().await?;
    Ok(IndexTemplate { todos })
}

#[derive(serde::Deserialize, Validate)]
pub struct NewTodo {
    #[validate(length(min = 1, max = 25))]
    pub description: String,
}

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

pub async fn rename_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
    ValidatedForm(todo): ValidatedForm<RenameTodo>,
) -> Result<impl IntoResponse, AppError> {
    database.rename_todo(id, todo.description).await?;
    Ok(Redirect::to("/"))
}

pub async fn toggle_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    database.toggle_todo(id).await?;
    Ok(Redirect::to("/"))
}

pub async fn delete_todo(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    database.delete_todo(id).await?;
    Ok(Redirect::to("/"))
}

pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}
