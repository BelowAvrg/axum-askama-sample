# TAAA Stack Todo List

This is a simple todo list application demonstrating the TAAA stack: Tailwind, Alpine.js, Axum, and Askama.

## Prerequisites

- Rust
- Docker
- PostgreSQL
- `sqlx-cli` (`cargo install sqlx-cli`)

## Setup

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/dallen/axum-askama-sample
    cd axum-askama-sample
    ```

2.  **Set up the database:**

    - Create a `.env` file with the following content:

      ```
      DATABASE_URL=postgres://user:password@localhost/todos
      ```

    - Create the database:

      ```bash
      sqlx database create
      ```

    - Run the migrations:

      ```bash
      sqlx migrate run
      ```

3.  **Run the application:**

    ```bash
    cargo run
    ```

    The application will be available at `http://localhost:3000`.

## Docker

You can also run the application using Docker:

1.  **Build the image:**

    ```bash
    docker build -t axum-askama-sample .
    ```

2.  **Run the container:**

    ```bash
    docker run -p 3000:3000 --env-file .env axum-askama-sample
    ```
