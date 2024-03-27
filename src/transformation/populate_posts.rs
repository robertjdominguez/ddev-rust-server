use crate::transformation::transform_md;
use std::error::Error;
use std::io;
use tokio::fs;

#[derive(Debug, Default, Clone)]
pub struct CardContent {
    pub slug: String,
    pub title: String,
    pub hook: String,
    pub image: String,
    pub created_at: String,
}

// We'll need a function that can get all the posts from the /posts directory
pub async fn get_posts(dir: String) -> Result<Vec<String>, io::Error> {
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
pub async fn read_file_and_create_card(filename: &str) -> Result<CardContent, Box<dyn Error>> {
    let file_path = format!("posts/{}", filename);
    let file_contents = fs::read_to_string(file_path).await?;

    let (frontmatter, _main_content) = transform_md::split_frontmatter_from_content(file_contents)
        .ok_or("No frontmatter found.")?;

    let card = frontmatter
        .lines()
        .fold(CardContent::default(), |mut card, line| {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "title" => card.title = value.trim().to_string(),
                    "hook" => card.hook = value.trim().to_string(),
                    "slug" => card.slug = value.trim().to_string(),
                    "created_at" => card.created_at = value.trim().to_string(),
                    "image" => card.image = value.trim().to_string(),
                    _ => log::warn!("Unexpected key in frontmatter: {}", key),
                }
            }
            card
        });

    Ok(card)
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
