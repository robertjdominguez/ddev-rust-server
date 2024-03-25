use chrono::{DateTime, NaiveDateTime, Utc};

/** This struct represents a single post and is reused from
* https://github.com/robertjdominguez/ddev-fetcher/blob/main/src/entities/posts.rs
*/
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub hook: String,
    pub image: String,
    pub slug: String,
}

#[derive(Clone)]
pub struct CardContent {
    pub slug: String,
    pub title: String,
    pub hook: String,
    pub image: String,
    pub created_at: String,
}

/*
* This function creates a new post based on inputs.
*/
pub fn create_post(
    title: String,
    body: String,
    created_at: String,
    image: String,
    hook: String,
    slug: String,
) -> Post {
    Post {
        id: 0,
        title,
        body,
        created_at,
        image,
        hook,
        slug,
    }
}

/** Here, we'll have an async function that takes in a single post
* and pulls out the relevant information. We need the slug (filename),
* title, hook, and image.
*/
pub async fn extract_card_information(post: Post) -> CardContent {
    CardContent {
        slug: post.slug,
        title: post.title,
        hook: post.hook,
        image: post.image,
        created_at: post.created_at,
    }
}

// This will parse our dates for us and we can use it in the re-shuffling algorithm below
fn parse_date(date_str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z").expect("Failed to parse date")
}

/** We'll also need a function to order our posts from newest to oldest
* by taking the created_at field for each and comparing them to each other.
*/
pub async fn order_cards(cards: &[CardContent]) -> Vec<CardContent> {
    let mut sorted_cards = cards.to_vec();

    // Sort cards from newest to oldest
    sorted_cards.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    sorted_cards
}
