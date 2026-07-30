use axum::{extract::Form, response::Html, response::IntoResponse};
use std::collections::HashMap;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, SmtpTransport,
    Transport, Message,
};

pub async fn submit_contact_form(Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    let name = form.get("name").unwrap_or(&"".to_string()).to_string();
    let email = form.get("email").unwrap_or(&"".to_string()).to_string();
    let message = form.get("message").unwrap_or(&"".to_string()).to_string();

    // Validate
    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Html(render_popup("❌", "Error!", "Please fill in all fields.", "#fff5f5", "#fcc", "#c00"));
    }

    if message.len() > 500 {
        return Html(render_popup("❌", "Error!", "Message is too long (max 500 characters).", "#fff5f5", "#fcc", "#c00"));
    }

    // Get password from environment
    let password = match std::env::var("GMAIL_APP_PASSWORD") {
        Ok(pwd) => pwd.replace(" ", ""),
        Err(e) => {
            eprintln!("❌ GMAIL_APP_PASSWORD not set: {}", e);
            return Html(render_popup("❌", "Error!", "Server configuration error. Please try again later.", "#fff5f5", "#fcc", "#c00"));
        }
    };

    // Send email
    match send_email(&name, &email, &message, &password) {
        Ok(_) => Html(render_popup("✅", "Message Sent!", "Thank you for reaching out! I'll get back to you soon.", "#f0faf0", "#b3ffb3", "#2d7a2d")),
        Err(e) => Html(render_popup("❌", "Error!", &format!("Failed to send: {}", e), "#fff5f5", "#fcc", "#c00")),
    }
}

fn render_popup(_icon: &str, title: &str, message: &str, bg_color: &str, border_color: &str, text_color: &str) -> String {
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

fn send_email(name: &str, email: &str, message: &str, password: &str) -> Result<(), String> {
    let smtp_username = "leemarkarojo7@gmail.com";

    // Parse email addresses
    let to = smtp_username.parse::<Mailbox>()
        .map_err(|e| format!("Invalid to email: {}", e))?;

    // 'from' must be your authenticated Gmail account to prevent SMTP 535 rejection
    let from = format!("Portfolio Contact <{}>", smtp_username).parse::<Mailbox>()
        .map_err(|e| format!("Invalid from email: {}", e))?;

    let reply_to = email.parse::<Mailbox>()
        .map_err(|e| format!("Invalid reply-to email: {}", e))?;

    let email_body = format!(
        "New contact form submission from your portfolio website:\n\n\
        Name: {}\n\
        Email: {}\n\
        \n\
        Message:\n{}",
        name, email, message
    );

    let email_msg = Message::builder()
        .from(from)
        .to(to)
        .reply_to(reply_to)
        .subject(format!("New message from {} via Portfolio", name))
        .body(email_body)
        .map_err(|e| format!("Failed to build email: {}", e))?;

    let creds = Credentials::new(smtp_username.to_string(), password.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| format!("Failed to connect to SMTP: {}", e))?
        .credentials(creds)
        .build();

    match mailer.send(&email_msg) {
        Ok(_) => {
            println!("✅ Email sent successfully from {} to {}", email, smtp_username);
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to send email: {}", e);
            Err(format!("Failed to send: {}", e))
        }
    }
}
