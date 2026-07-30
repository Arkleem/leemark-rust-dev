// src/handlers.rs
use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Json, sse::{Event, Sse}},
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::Stream;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tokio::net::TcpListener;
use maud::html;

use crate::state::AppState;
use crate::home::view::home_page;
use crate::contact::handler::submit_contact_form;

/// Home page handler
pub async fn get_home(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(home_page(&state).into_string())
}

/// Scroll Progress Handler
pub async fn scroll_progress_updates(
    Query(params): Query<HashMap<String, String>>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let pct = params
        .get("pct")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
        .min(100)
        .max(0);

    let html_fragment = render_progress_bar(pct).into_string();
    let sse_data = format!("elements {}", html_fragment.replace('\n', " "));

    let event = Event::default()
        .event("datastar-patch-elements")
        .data(sse_data);

    let stream = tokio_stream::once(Ok(event));
    Sse::new(stream)
}

/// Starfield Update Handler
pub async fn starfield_updates(
    Query(params): Query<HashMap<String, String>>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let x = params.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(85);
    let y = params.get("y").and_then(|s| s.parse::<i32>().ok()).unwrap_or(34);
    let speed = params.get("speed").and_then(|s| s.parse::<i32>().ok()).unwrap_or(94);

    let html_fragment = render_starfield_fragment(x, y, speed).into_string();
    let sse_data = format!("elements {}", html_fragment.replace('\n', " "));

    let event = Event::default()
        .event("datastar-patch-elements")
        .data(sse_data);

    let stream = tokio_stream::once(Ok(event));
    Sse::new(stream)
}

/// Render progress bar HTML
fn render_progress_bar(percent: i32) -> maud::Markup {
    let circumference = 565.48;
    let dash_offset = circumference * (1.0 - (percent as f64 / 100.0));
    let display_percent = percent.min(100).max(0);

    html! {
        div id="progress-bar" {
            svg
                width="70"
                height="70"
                viewBox="-25 -25 250 250"
                style="transform: rotate(-90deg)"
            {
                circle
                    r="90"
                    cx="100"
                    cy="100"
                    fill="transparent"
                    stroke="rgba(255,255,255,0.15)"
                    stroke-width="16px"
                    stroke-dasharray="565.48px"
                    stroke-dashoffset="0px"
                {}
                circle
                    r="90"
                    cx="100"
                    cy="100"
                    fill="transparent"
                    stroke="#d4af37"
                    stroke-width="16px"
                    stroke-linecap="round"
                    stroke-dasharray="565.48px"
                    stroke-dashoffset=(format!("{:.2}px", dash_offset))
                {}
                text
                    x="44px"
                    y="115px"
                    fill="#d4af37"
                    font-size="52px"
                    font-weight="bold"
                    style="transform: rotate(90deg) translate(0px, -196px); font-family: 'EB Garamond', 'Garamond', serif;"
                {
                    (format!("{}%", display_percent))
                }
            }
        }
    }
}

/// Render starfield HTML fragment
fn render_starfield_fragment(x: i32, y: i32, speed: i32) -> maud::Markup {
    html! {
        star-field
            data-attr:center-x=(x)
            data-attr:center-y=(y)
            data-attr:speed=(speed)
            style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;"
        {}
    }
}

/// Favicon handler
pub async fn favicon() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

/// Create the application router
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/contact", post(submit_contact_form))
        .route("/scroll-progress", get(scroll_progress_updates))
        .route("/starfield-update", get(starfield_updates))
        .route("/favicon.ico", get(favicon))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Start the server
pub async fn start_server(router: Router) -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://0.0.0.0:3000");
    axum::serve(listener, router).await?;
    Ok(())
}
