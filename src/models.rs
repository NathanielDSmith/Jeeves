use serde::{Deserialize, Serialize};

// Core data model for job listings.
// Derive macros auto-generate trait implementations:
// - Serialize/Deserialize: JSON encoding/decoding via serde
// - Debug: printable representation for debugging
// - Clone: explicit copying (memcpy-like, not deep clone)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub category: String,
    pub applicants: u32,
    pub url: String,
    #[serde(default)]
    pub description: String,
}

impl Job {
    pub fn new(id: String, title: String, company: String, category: String, applicants: u32, url: String) -> Self {
        Job {
            id,
            title,
            company,
            category,
            applicants,
            url,
            description: String::new(),
        }
    }

    pub fn display(&self) {
        println!("  {} | {} @ {} ({} applicants)", self.category, self.title, self.company, self.applicants);
    }
}
