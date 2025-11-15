use crate::models::{Post, PostCard, Project, ProjectCard};
use crate::utils::{split_frontmatter_from_content, parse_yaml_frontmatter, extract_string_field, extract_bool_field, extract_string_array_field};
use pulldown_cmark;
use std::error::Error;
use std::fs;
use std::io;
use tokio::fs as async_fs;

pub async fn get_content_files(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut file_names = Vec::new();
    let mut entries = async_fs::read_dir(dir).await?;

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

pub fn markdown_to_html(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

pub async fn create_post_card(filename: &str) -> Result<PostCard, Box<dyn Error>> {
    let file_path = format!("posts/{}", filename);
    let file_contents = async_fs::read_to_string(file_path).await?;

    let (frontmatter_str, _main_content) = split_frontmatter_from_content(file_contents)
        .ok_or("No frontmatter found.")?;

    let frontmatter = parse_yaml_frontmatter(&frontmatter_str)?;

    Ok(PostCard {
        slug: extract_string_field(&frontmatter, "slug").unwrap_or_default(),
        title: extract_string_field(&frontmatter, "title").unwrap_or_default(),
        hook: extract_string_field(&frontmatter, "hook").unwrap_or_default(),
        image: extract_string_field(&frontmatter, "image").unwrap_or_default(),
        created_at: extract_string_field(&frontmatter, "created_at").unwrap_or_default(),
    })
}

pub async fn create_post_from_file(filename: &str) -> Result<Post, Box<dyn Error>> {
    let file_path = format!("posts/{}", filename);
    let file_contents = async_fs::read_to_string(file_path).await?;

    let (frontmatter_str, main_content) = split_frontmatter_from_content(file_contents)
        .ok_or("No frontmatter found.")?;

    let frontmatter = parse_yaml_frontmatter(&frontmatter_str)?;
    let html_content = markdown_to_html(&main_content);

    Ok(Post {
        title: extract_string_field(&frontmatter, "title").unwrap_or_default(),
        hook: extract_string_field(&frontmatter, "hook").unwrap_or_default(),
        slug: extract_string_field(&frontmatter, "slug").unwrap_or_default(),
        created_at: extract_string_field(&frontmatter, "created_at").unwrap_or_default(),
        image: extract_string_field(&frontmatter, "image").unwrap_or_default(),
        content: Some(html_content),
    })
}

pub async fn create_project_card(filename: &str) -> Result<ProjectCard, Box<dyn Error>> {
    let file_path = format!("projects/{}", filename);
    let file_contents = async_fs::read_to_string(file_path).await?;

    let (frontmatter_str, _main_content) = split_frontmatter_from_content(file_contents)
        .ok_or("No frontmatter found.")?;

    let frontmatter = parse_yaml_frontmatter(&frontmatter_str)?;

    Ok(ProjectCard {
        slug: extract_string_field(&frontmatter, "slug").unwrap_or_default(),
        title: extract_string_field(&frontmatter, "title").unwrap_or_default(),
        description: extract_string_field(&frontmatter, "description").unwrap_or_default(),
        image: extract_string_field(&frontmatter, "image").unwrap_or_default(),
        tech_stack: extract_string_array_field(&frontmatter, "tech_stack"),
        created_at: extract_string_field(&frontmatter, "created_at").unwrap_or_default(),
        featured: extract_bool_field(&frontmatter, "featured"),
    })
}

pub async fn create_project_from_file(filename: &str) -> Result<Project, Box<dyn Error>> {
    let file_path = format!("projects/{}", filename);
    let file_contents = async_fs::read_to_string(file_path).await?;

    let (frontmatter_str, main_content) = split_frontmatter_from_content(file_contents)
        .ok_or("No frontmatter found.")?;

    let frontmatter = parse_yaml_frontmatter(&frontmatter_str)?;
    let html_content = markdown_to_html(&main_content);

    Ok(Project {
        title: extract_string_field(&frontmatter, "title").unwrap_or_default(),
        description: extract_string_field(&frontmatter, "description").unwrap_or_default(),
        slug: extract_string_field(&frontmatter, "slug").unwrap_or_default(),
        created_at: extract_string_field(&frontmatter, "created_at").unwrap_or_default(),
        image: extract_string_field(&frontmatter, "image").unwrap_or_default(),
        tech_stack: extract_string_array_field(&frontmatter, "tech_stack"),
        url: extract_string_field(&frontmatter, "url"),
        github: extract_string_field(&frontmatter, "github"),
        featured: extract_bool_field(&frontmatter, "featured"),
        content: Some(html_content),
    })
}

pub fn order_post_cards(cards: &[PostCard]) -> Vec<PostCard> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    sorted_cards
}

pub fn order_project_cards(cards: &[ProjectCard]) -> Vec<ProjectCard> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    sorted_cards
}