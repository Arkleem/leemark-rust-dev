use crate::state::AppState;
use crate::models::ContactForm;  // ← Changed from handler to models
use maud::{html};

pub struct HomeService {
    state: AppState,
}

impl HomeService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn render_home_page(&self) -> String {
        use crate::components::layout::base_layout;
        use crate::hero::view::hero_section;
        use crate::summary::view::summary_section;
        use crate::skills::view::skills_section;
        use crate::projects::view::projects_section;
        use crate::experience::view::experience_section;
        use crate::contact::view::contact_section;

        let content = html! {
            (hero_section())
            (summary_section())
            (skills_section(&self.state))
            (projects_section(&self.state))
            (experience_section(&self.state))
            (contact_section())
        };

        base_layout("Leemark Arojo - Portfolio", content).into_string()
    }

    pub async fn process_contact(&self, form: ContactForm) -> Result<String, String> {
        // Validate
        if form.name.len() < 2 {
            return Err("Name must be at least 2 characters".to_string());
        }

        let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if !email_regex.is_match(&form.email) {
            return Err("Please enter a valid email".to_string());
        }

        if form.message.len() < 10 {
            return Err("Message must be at least 10 characters".to_string());
        }

        if form.message.len() > 500 {
            return Err("Message cannot exceed 500 characters".to_string());
        }

        tracing::info!(
            "Contact form submission from: {} ({})",
            form.name,
            form.email
        );
        tracing::debug!("Message: {}", form.message);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok("Thank you for your message! I'll get back to you soon.".to_string())
    }
}
