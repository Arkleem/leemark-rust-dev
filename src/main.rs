// src/main.rs
mod models;
mod state;
mod handlers;
mod components;
mod home;
mod hero;
mod summary;
mod skills;
mod projects;
mod experience;
mod contact;

use handlers::create_router;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();


    tracing_subscriber::fmt::init();

    let state = AppState::new();
    let router = create_router(state);

    handlers::start_server(router).await?;

    Ok(())
}
