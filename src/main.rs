// src/main.rs
#![allow(dead_code)]
#![allow(unused_imports)]

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
use shuttle_axum::AxumService;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenvy::dotenv().ok();

    match std::env::var("GMAIL_APP_PASSWORD") {
        Ok(pwd) => println!("✅ Password loaded: {} characters", pwd.len()),
        Err(e) => println!("⚠️ Password not loaded: {}", e),
    }

    let state = AppState::new();
    let router = create_router(state);

    // Wrap with AxumService explicitly
    Ok(AxumService(router).into())
}
