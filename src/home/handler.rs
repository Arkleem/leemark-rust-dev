use axum::{
    extract::{State, Query},  // ✅ Add Query here
    response::{Json, sse::{Event, Sse}},  // Remove KeepAlive if not used
    http::StatusCode,
};
use std::convert::Infallible;
use std::collections::HashMap;  // ✅ Add HashMap
use tokio_stream::Stream;
use maud::html;
use crate::state::AppState;
use crate::home::service::HomeService;
use crate::models::ContactForm;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_contact(
    State(state): State<AppState>,
    Json(form): Json<ContactForm>,
) -> (StatusCode, Json<ContactResponse>) {
    let service = HomeService::new(state);

    match service.process_contact(form).await {
        Ok(message) => (
            StatusCode::OK,
            Json(ContactResponse {
                success: true,
                message,
            })
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ContactResponse {
                success: false,
                message: err,
            })
        ),
    }
}

// ✅ Scroll Progress Endpoint - receives pct from query
pub async fn scroll_progress_updates(
    Query(params): Query<HashMap<String, String>>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Get the percentage from the query params
    let pct = params
        .get("pct")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
        .min(100)
        .max(0);

    // Render the progress bar HTML with the current percentage
    let html_fragment = render_progress_bar(pct).into_string();
    let sse_data = format!("elements {}", html_fragment.replace('\n', " "));

    let event = Event::default()
        .event("datastar-patch-elements")
        .data(sse_data);

    let stream = tokio_stream::once(Ok(event));
    Sse::new(stream)
}

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
                // Background circle
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
                // Progress circle
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
                // Percentage text - ONLY THIS, no extra text below
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
