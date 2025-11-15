use actix_web::{get, web, HttpResponse, Responder};
use crate::models::{Post, PostCard};
use crate::services::{get_content_files, create_post_card, create_post_from_file, order_post_cards};
use chrono::DateTime;
use std::fs;

#[get("/posts")]
pub async fn posts() -> impl Responder {
    let html_wrapper = match fs::read_to_string("templates/posts.html") {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading HTML wrapper: {}", e))
        }
    };

    let posts = match get_content_files("posts").await {
        Ok(posts) => posts,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching posts: {}", e))
        }
    };

    let mut cards = Vec::new();
    for post in posts {
        match create_post_card(&post).await {
            Ok(card) => cards.push(card),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error processing a post: {}", e))
            }
        }
    }

    let card_template = match fs::read_to_string("templates/card.html") {
        Ok(template) => template,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading card template: {}", e))
        }
    };

    let cards = order_post_cards(&cards);
    let cards_html: String = cards
        .iter()
        .map(|card| {
            let formatted_date = match DateTime::parse_from_rfc3339(&card.created_at) {
                Ok(datetime) => datetime.format("%B %d, %Y %H:%M").to_string(),
                Err(_) => "Invalid date".to_string(),
            };

            card_template
                .replace("{title}", &card.title.replace("\"", ""))
                .replace("{date}", &formatted_date)
                .replace("{hook}", &card.hook.replace("\"", ""))
                .replace("{slug}", &card.slug)
                .replace("{image}", &card.image)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let final_html = html_wrapper.replace("{}", &cards_html);
    HttpResponse::Ok().body(final_html)
}

pub async fn show_post(slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();
    let filename = format!("{}.md", slug);
    
    match create_post_from_file(&filename).await {
        Ok(post) => {
            let template = match fs::read_to_string("templates/post.html") {
                Ok(template) => template,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Error reading post template: {}", e))
                }
            };

            let html_content = template
                .replace("{page_title}", &post.title)
                .replace("{og_title}", &post.title)
                .replace("{hook}", &post.hook)
                .replace("{og_description}", &post.hook)
                .replace("{og_image}", &post.image)
                .replace("{slug}", &post.slug)
                .replace("{post_image}", &post.image)
                .replace("{post_content}", &post.content.unwrap_or_default());

            HttpResponse::Ok().body(html_content)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error loading post: {}", e))
        }
    }
}