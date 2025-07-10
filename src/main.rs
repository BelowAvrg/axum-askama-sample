use axum_askama_sample::{database::Database, routes::todos::create_router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
