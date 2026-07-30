// src/handlers.rs
use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Json, sse::{Event, Sse}},
    routing::{get, post},
    Router,
    Json as AxumJson,
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
use crate::models::ContactForm;
use crate::contact::handler::submit_contact_form;

/// Home page handler
pub async fn get_home(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(home_page(&state).into_string())
}

/// Contact form handler - accepts JSON only
pub async fn submit_contact(
    State(_state): State<Arc<AppState>>,
    AxumJson(form): AxumJson<ContactForm>,
) -> impl IntoResponse {
    println!("📧 Contact form submission (JSON):");
    println!("  Name: {}", form.name);
    println!("  Email: {}", form.email);
    println!("  Message: {}", form.message);

    // Validate form
    if form.name.is_empty() || form.email.is_empty() || form.message.is_empty() {
        let response = serde_json::json!({
            "success": false,
            "message": "All fields are required."
        });
        return Json(response);
    }

    if form.message.len() > 500 {
        let response = serde_json::json!({
            "success": false,
            "message": "Message is too long (max 500 characters)."
        });
        return Json(response);
    }

    // Get password from environment
    let password = match std::env::var("GMAIL_APP_PASSWORD") {
        Ok(pwd) => pwd.replace(" ", ""),
        Err(e) => {
            eprintln!("❌ GMAIL_APP_PASSWORD not set: {}", e);
            let response = serde_json::json!({
                "success": false,
                "message": "Server configuration error. Please try again later."
            });
            return Json(response);
        }
    };
    match std::env::var("GMAIL_APP_PASSWORD") {
        Ok(pwd) => println!("✅ Password loaded: {} characters", pwd.len()),
        Err(_) => println!("❌ Password NOT loaded"),
    }

    // Send email (sync version)
    match send_email(&form.name, &form.email, &form.message, &password) {
        Ok(_) => {
            let response = serde_json::json!({
                "success": true,
                "message": format!("Thank you {}! Your message has been sent successfully.", form.name)
            });
            Json(response)
        }
        Err(e) => {
            eprintln!("❌ Failed to send email: {}", e);
            let response = serde_json::json!({
                "success": false,
                "message": format!("Failed to send message: {}", e)
            });
            Json(response)
        }
    }
}

/// Send email using Gmail SMTP (SYNC version - no async)
fn send_email(name: &str, email: &str, message: &str, password: &str) -> Result<(), String> {
    use lettre::{
        message::Mailbox,
        transport::smtp::authentication::Credentials,
        SmtpTransport,
        Transport,
        Message,
    };

    let smtp_username = "leemarkarojo7@gmail.com";

    // Parse email addresses
    let to = "leemarkarojo7@gmail.com".parse::<Mailbox>()
        .map_err(|e| format!("Invalid to email: {}", e))?;

    let from = format!("Portfolio Contact <{}>", smtp_username).parse::<Mailbox>()
        .map_err(|e| format!("Invalid from email: {}", e))?;

    // Build email body
    let email_body = format!(
        "New contact form submission from your portfolio website:\n\n\
        ──────────────────────────────\n\
        📝 Name: {}\n\
        📧 Email: {}\n\
        ──────────────────────────────\n\n\
        📨 Message:\n{}\n\n\
        ──────────────────────────────\n\
        Sent from: {}",
        name, email, message, email
    );

    let email_msg = Message::builder()
        .from(from)
        .to(to)
        .reply_to(email.parse().map_err(|e| format!("Invalid reply-to email: {}", e))?)
        .subject(format!("[Portfolio] New message from {} ({})", name, email))
        .body(email_body)
        .map_err(|e| format!("Failed to build email: {}", e))?;

    // Create SMTP credentials
    let creds = Credentials::new(smtp_username.to_string(), password.to_string());

    // Create SMTP transport
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| format!("SMTP connection failed: {}", e))?
        .credentials(creds)
        .build();

    // Send email (sync)
    match mailer.send(&email_msg) {
        Ok(_) => {
            println!("✅ Email sent successfully from {} to {}", email, "leemarkarojo7@gmail.com");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to send email: {}", e);
            Err(format!("Failed to send email: {}", e))
        }
    }
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
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, router).await?;
    Ok(())
}
