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

use handlers::{create_router, start_server};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Debug: Check if password is loaded
    match std::env::var("GMAIL_APP_PASSWORD") {
        Ok(pwd) => println!("✅ Password loaded: {} characters", pwd.len()),
        Err(e) => println!("⚠️ Password not loaded: {}", e),
    }

    tracing_subscriber::fmt::init();

    let state = AppState::new();
    let router = create_router(state);

    start_server(router).await?;

    Ok(())
}
