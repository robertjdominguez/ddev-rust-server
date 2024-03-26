use crate::transformation::transform_md;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::io;
use tokio::fs;

#[derive(Clone)]
pub struct CardContent {
    pub slug: &str,
    pub title: &str,
    pub hook: &str,
    pub image: &str,
    pub created_at: &str,
}

// We'll need a function that can get all the posts from the /posts directory
pub async fn get_posts(dir: String) -> Result<Vec<String>, io::Error> {
    log::debug!("Here!");
    let mut file_names = Vec::new();
    let mut entries = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        if entry_path.is_file() {
            if let Some(name) = entry_path.file_name() {
                if let Some(name_str) = name.to_str() {
                    file_names.push(name_str.to_owned());
                }
            }
        }
    }
    Ok(file_names)
}

/** After we have a file's name, we can then read the file and create a Post type for it
* to then loop over and generate a card.
*/
pub async fn read_file_and_create_post(filename: &str) -> Result<String, String> {
    let file_path = format!("posts/{}", filename);
    let file = tokio::fs::read_to_string(file_path).await;

    let file = match file {
        Ok(file) => file,
        Err(e) => return Err(format!("Error reading file: {}", e)),
    };

    // Use the splitter function from transform_md to get the frontmatter
    let split_contents = transform_md::split_frontmatter_from_content(file);

    match split_contents {
        Some((frontmatter, _main_content)) => {
            // We'll split frontmatter by each line and then by colons

            for (index, line) in frontmatter.lines().enumerate() {
                let mut card = CardContent {
                    title: "Sample",
                    hook: "Sample",
                    image: "Sample",
                    slug: "Sample",
                    created_at: "Sample",
                };

                let mut values = line.split_once(":");
                match values {
                    Some((key, value)) => {
                        if index == 0 {
                            card.title = value;
                        }
                        log::debug!("{:?}", key);
                    }
                    None => log::debug!("Well, shit"),
                }
            }

            Ok(frontmatter)
        }
        None => Err("No frontmatter found.".to_string()),
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
