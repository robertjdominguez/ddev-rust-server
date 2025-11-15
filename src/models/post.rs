use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub hook: String,
    pub slug: String,
    pub created_at: String,
    pub image: String,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCard {
    pub slug: String,
    pub title: String,
    pub hook: String,
    pub image: String,
    pub created_at: String,
}