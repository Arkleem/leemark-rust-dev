use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub images: Option<Vec<String>>,
    pub full_description: Option<String>,
    pub image: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub github_link: Option<String>,
    pub features: Option<Vec<String>>,
    pub demo_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub period: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub location: String,
    pub period: String,
    pub major: Option<String>,
    pub achievement: Option<String>,
    pub coursework: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}
