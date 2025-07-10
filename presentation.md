---
title: Super-Simple Webapps with Rust
author: Dallen "BelowAvrg" Petersen
theme:
  name: tokyonight-storm
options:
  incremental_lists: true
---

<!-- font_size: 3 -->
I Have A Dream
---
- Build simple web apps quickly
- Have them run forever with minimal maintenance
- Make them easy to scale in the worst case scenario (I actually get users)

<!-- end_slide -->
<!-- font_size: 3 -->
A Little About My Life
---
- I have very little free time to write code
  - even less to maintain it
- I will often go long periods of time between working on a project

<!-- end_slide -->
<!-- font_size: 3 -->
The Stack
---
- Axum - HTTP Routing
- Askama - HTML Templating
  - Optionally with Tailwind/Alpine/HTMX
- Postgres (using sqlx) - Database

<!-- end_slide -->
<!-- font_size: 3 -->
Why Rust?
---
- ~Memory~ ~Saftey~
- ~Performance~
- Type system

# Compile Time Verification

<!-- end_slide -->
<!-- font_size: 3 -->
Axum
---
- Part of the tokio project, so it's well supported
- Leans heavily into core crates in the tokio/rust ecosystem
- Still not in 1.0, so there are breaking changes with each major release
  - Consider Actix for something more established

<!-- end_slide -->
<!-- font_size: 3 -->
Askama
---
- Type-safe compiler inspired by Jinja
- Your templates are compiled directly into your binary

<!-- end_slide -->
<!-- font_size: 3 -->
Postgres/Sqlx
---
- Sqlx provides compile-time database schema/type verification
- Postgres allows independent database scaling in case you accidentally make something people like


<!-- end_slide -->
<!-- font_size: 3 -->
Example Time!
---
- https://github.com/BelowAvrg/axum-askama-sample
<!-- newline -->
- https://belowavrg.com
<!-- newline -->
- https://ratemyreads.com

<!-- end_slide -->
<!-- font_size: 2 -->
Creating the App
---
``` rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database = Database::new().await?;

    let app = create_router(database);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
```

<!-- end_slide -->
<!-- font_size: 2 -->
Routes
---
``` rust
pub fn create_router(database: Database) -> Router {
    Router::new()
        .route("/", get(index).post(add_todo))
        .route("/toggle/:id", post(toggle_todo))
        .route("/delete/:id", post(delete_todo))
        .route("/rename/:id", post(rename_todo))
        .with_state(database)
}
```
<!-- pause -->
``` rust
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<Todo>,
}

pub async fn index(State(database): State<Database>) -> Result<impl IntoResponse, AppError> {
    let todos = database.get_todos().await?;
    Ok(IndexTemplate { todos })
}
```

<!-- end_slide -->
<!-- font_size: 2 -->
Askama syntax
---
### Constants - If Statements - Values
```
{% set value = 4 %}
{% if value > crate::MAX_NB_USERS %}
    <p>{{ value }} is bigger than MAX_NB_USERS.</p>
{% else %}
    <p>{{ value }} is less than MAX_NB_USERS.</p>
{% endif %}
```

### Loops
```
{% for todo in todos %}
  html...
{% endfor %}
```

<!-- end_slide -->
<!-- font_size: 2 -->
Post Deserialization With Serde
---
``` rust
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
```

<!-- end_slide -->
<!-- font_size: 2 -->
Compile-time Verified Queries
---
### Rust Type
``` rust
#[derive(sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub is_done: bool,
}
```

### Postgres Table
``` sql
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT FALSE
);
```

<!-- end_slide -->
<!-- font_size: 2 -->
Compile-time Verified Queries
---
``` rust
#[derive(sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub is_done: bool,
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
```

<!-- end_slide -->
<!-- font_size: 2 -->
Compile-time Verified Queries
---
``` rust
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
```

<!-- end_slide -->
<!-- font_size: 3 -->
Stack Summary
---
### When is it good?
- Simple, multi-page applications with limited reactivity required

##  When is it bad?
- You want a job
- You have a job
- You need lots of reactivity on the frontend

<!-- end_slide -->
<!-- font_size: 3 -->
Bonus Tips!
---
### Axum
- You can use the `#[debug_handler]` macro above your route handlers to get better error messages for debugging

### Sqlx
- Set SQLX_OFFLINE=true in your build pipeline so you don't need to run a DB to compile agaist
  - Don't forget to run cargo sqlx-prepare

<!-- end_slide -->
<!-- font_size: 3 -->
Bonus Tips!
---
### Frontend
- You can use AlpineJS and Tailwind to simplify frontend development
- You don't even need a build step to compile
``` html
<script src="https://cdn.tailwindcss.com"></script>
<script src="//unpkg.com/alpinejs" defer></script>
```

<!-- end_slide -->
<!-- font_size: 3 -->
Bonus Tips!
---
### Frontend
- You can use cargo-watch to automatically recompile
``` bash
cargo watch -x run
```

- Compile the askama macros with higer optimization in dev
``` toml
[profile.dev.package.askama_derive]
opt-level = 3
```