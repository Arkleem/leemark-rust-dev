// src/contact/handler.rs
use axum::{extract::Form, response::Html, response::IntoResponse};
use std::collections::HashMap;
use reqwest::Client;

pub async fn submit_contact_form(Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    let name = form.get("name").unwrap_or(&"".to_string()).to_string();
    let email = form.get("email").unwrap_or(&"".to_string()).to_string();
    let message = form.get("message").unwrap_or(&"".to_string()).to_string();

    // Validate
    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Html(render_popup("Error!", "Please fill in all fields.", "#fff5f5", "#fcc", "#c00"));
    }

    if message.len() > 500 {
        return Html(render_popup("Error!", "Message is too long (max 500 characters).", "#fff5f5", "#fcc", "#c00"));
    }

    // Get Resend API key from environment
    let api_key = match std::env::var("RESEND_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("❌ RESEND_API_KEY not set: {}", e);
            return Html(render_popup("Error!", "Server configuration error. Please try again later.", "#fff5f5", "#fcc", "#c00"));
        }
    };

    // Send email via Resend
    match send_email_via_resend(&name, &email, &message, &api_key).await {
        Ok(_) => Html(render_popup("Message Sent!", "Thank you for reaching out! I'll get back to you soon.", "#f0faf0", "#b3ffb3", "#2d7a2d")),
        Err(e) => {
            eprintln!("❌ Failed to send email: {}", e);
            Html(render_popup("Error!", &format!("Failed to send: {}", e), "#fff5f5", "#fcc", "#c00"))
        }
    }
}

async fn send_email_via_resend(name: &str, email: &str, message: &str, api_key: &str) -> Result<(), String> {
    let client = Client::new();

    // Resend default sandbox email (or use your verified domain)
    // For production, verify a domain at: https://resend.com/domains
    let from_email = "onboarding@resend.dev"; // Default sandbox
    // When you verify a domain, change to: "portfolio@yourdomain.com"

    let response = client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "from": from_email,
            "to": ["leemarkarojo7@gmail.com"],
            "reply_to": email,
            "subject": format!("[Portfolio] New message from {}", name),
            "text": format!(
                "New contact form submission from your portfolio website:\n\n\
                Name: {}\n\
                Email: {}\n\n\
                Message:\n{}",
                name, email, message
            ),
            "html": format!(
                "<h2>New Contact Form Submission</h2>\
                <p><strong>Name:</strong> {}</p>\
                <p><strong>Email:</strong> {}</p>\
                <p><strong>Message:</strong></p>\
                <p>{}</p>",
                name, email, message.replace("\n", "<br>")
            )
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        println!("✅ Email sent via Resend from {} to {}", email, "leemarkarojo7@gmail.com");
        Ok(())
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Resend API error: {}", error_text))
    }
}

fn render_popup(title: &str, message: &str, bg_color: &str, border_color: &str, text_color: &str) -> String {
    format!(
        r#"
        <div style="
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 10000;
            padding: 35px 45px;
            border-radius: 16px;
            max-width: 450px;
            width: 90%;
            text-align: center;
            font-family: 'EB Garamond', 'Garamond', serif;
            background: {};
            border: 2px solid {};
            box-shadow: 0 25px 80px rgba(0,0,0,0.4);
            animation: popupFadeIn 0.3s ease;
        ">
            <div style="font-size: 52px; margin-bottom: 12px;"></div>
            <h3 style="color: {}; font-size: 1.5rem; margin: 0 0 8px 0; font-weight: 700;">{}</h3>
            <p style="color: {}; font-size: 1rem; margin: 0 0 25px 0; line-height: 1.6;">{}</p>
            <a
                href="/#contact"
                style="
                    display: inline-block;
                    padding: 10px 40px;
                    background: linear-gradient(to right, #d4af37, #b8960f);
                    color: white;
                    border: none;
                    border-radius: 30px;
                    font-size: 1rem;
                    cursor: pointer;
                    font-weight: 600;
                    text-decoration: none;
                    font-family: 'EB Garamond', 'Garamond', serif;
                    letter-spacing: 0.5px;
                    transition: all 0.3s ease;
                "
                onmouseover="this.style.transform = 'scale(1.05)'; this.style.boxShadow = '0 4px 20px rgba(212, 175, 55, 0.4)';"
                onmouseout="this.style.transform = 'scale(1)'; this.style.boxShadow = 'none';"
            >
                Close
            </a>
        </div>
        <div style="
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: 9999;
            background: rgba(0, 0, 0, 0.5);
            backdrop-filter: blur(6px);
        " onclick="this.remove(); document.querySelector('[style*=\'position: fixed; top: 50%; left: 50%;\']').remove();"></div>
        <style>
            @keyframes popupFadeIn {{
                from {{
                    opacity: 0;
                    transform: translate(-50%, -50%) scale(0.9);
                }}
                to {{
                    opacity: 1;
                    transform: translate(-50%, -50%) scale(1);
                }}
            }}
        </style>
        "#,
        bg_color, border_color, text_color, title, text_color, message
    )
}
