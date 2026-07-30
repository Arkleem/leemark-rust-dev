// src/state.rs
use std::sync::Arc;
use std::fs;
use crate::models::{Project, Skill, Experience, Education};

#[derive(Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            projects: Self::load_projects(),
            skills: Self::skills(),
            experiences: Self::load_experiences(),
            education: Self::load_education(),
        })
    }

    fn load_projects() -> Vec<Project> {
        let path = "projects.json";
        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Project>>(&content) {
                    Ok(projects) => {
                        println!("✅ Loaded {} projects from {}", projects.len(), path);
                        projects
                    }
                    Err(e) => {
                        eprintln!("❌ Error parsing {}: {}", path, e);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", path, e);
                Vec::new()
            }
        }
    }

    fn load_experiences() -> Vec<Experience> {
        let path = "experiences.json";
        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Experience>>(&content) {
                    Ok(experiences) => {
                        println!("✅ Loaded {} experiences from {}", experiences.len(), path);
                        experiences
                    }
                    Err(e) => {
                        eprintln!("❌ Error parsing {}: {}", path, e);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", path, e);
                Vec::new()
            }
        }
    }

    fn load_education() -> Vec<Education> {
        let path = "education.json";
        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Education>>(&content) {
                    Ok(education) => {
                        println!("✅ Loaded {} education entries from {}", education.len(), path);
                        education
                    }
                    Err(e) => {
                        eprintln!("❌ Error parsing {}: {}", path, e);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", path, e);
                Vec::new()
            }
        }
    }

    fn skills() -> Vec<Skill> {
        vec![
            Skill { name: "Rust".to_string(), icon: "fab fa-rust".to_string(), category: "backend".to_string() },
            Skill { name: "Axum".to_string(), icon: "fas fa-bolt".to_string(), category: "backend".to_string() },
            Skill { name: "Maud".to_string(), icon: "fas fa-file-code".to_string(), category: "backend".to_string() },
            Skill { name: "HTML/CSS".to_string(), icon: "fas fa-code".to_string(), category: "frontend".to_string() },
            Skill { name: "Datastar".to_string(), icon: "fas fa-star".to_string(), category: "frontend".to_string() },
            Skill { name: "encre-css".to_string(), icon: "fas fa-palette".to_string(), category: "frontend".to_string() },
            Skill { name: "SurrealDB".to_string(), icon: "fas fa-database".to_string(), category: "database".to_string() },
            Skill { name: "SurrealQL".to_string(), icon: "fas fa-terminal".to_string(), category: "database".to_string() },
            Skill { name: "SurrealKV".to_string(), icon: "fas fa-server".to_string(), category: "database".to_string() },
            Skill { name: "Linux (Fedora)".to_string(), icon: "fab fa-linux".to_string(), category: "deployment".to_string() },
            Skill { name: "Railway".to_string(), icon: "fas fa-cloud".to_string(), category: "deployment".to_string() },
            Skill { name: "DigitalOcean".to_string(), icon: "fab fa-digital-ocean".to_string(), category: "deployment".to_string() },
            Skill { name: "Tauri (Desktop)".to_string(), icon: "fas fa-desktop".to_string(), category: "crossplatform".to_string() },
            Skill { name: "Tauri (Mobile)".to_string(), icon: "fas fa-mobile-alt".to_string(), category: "crossplatform".to_string() },
            Skill { name: "WebViews".to_string(), icon: "fas fa-globe".to_string(), category: "crossplatform".to_string() },
        ]
    }
}
