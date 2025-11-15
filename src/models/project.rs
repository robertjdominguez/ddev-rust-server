use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub slug: String,
    pub created_at: String,
    pub image: String,
    pub tech_stack: Vec<String>,
    pub url: Option<String>,
    pub github: Option<String>,
    pub featured: bool,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCard {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub tech_stack: Vec<String>,
    pub created_at: String,
    pub featured: bool,
}