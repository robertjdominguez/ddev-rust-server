use actix_web::{get, web, HttpResponse, Responder};
use crate::models::{ProjectCard};
use crate::services::{get_content_files, create_project_card, order_project_cards};
use chrono::DateTime;
use std::fs;

#[get("/projects")]
pub async fn projects() -> impl Responder {
    let html_wrapper = match fs::read_to_string("templates/projects.html") {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading HTML wrapper: {}", e))
        }
    };

    let projects = match get_content_files("projects").await {
        Ok(projects) => projects,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching projects: {}", e))
        }
    };

    let mut cards = Vec::new();
    for project in projects {
        match create_project_card(&project).await {
            Ok(card) => cards.push(card),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error processing a project: {}", e))
            }
        }
    }

    // Show empty state if no projects
    if cards.is_empty() {
        let empty_state = r#"
        <div class="empty-state">
            <div class="empty-icon">🚧</div>
            <h3>Projects coming soon!</h3>
            <p>I'm working on some exciting projects. Check back soon to see what I've been building.</p>
            <p>In the meantime, feel free to check out my <a href="/posts">blog posts</a> or <a href="https://github.com/robertjdominguez" target="_blank">GitHub</a>.</p>
        </div>
        "#;
        let final_html = html_wrapper.replace("{}", empty_state);
        return HttpResponse::Ok().body(final_html);
    }

    let card_template = match fs::read_to_string("templates/project-card.html") {
        Ok(template) => template,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading project card template: {}", e))
        }
    };

    let cards = order_project_cards(&cards);
    let cards_html: String = cards
        .iter()
        .map(|card| {
            let formatted_date = match DateTime::parse_from_rfc3339(&card.created_at) {
                Ok(datetime) => datetime.format("%B %d, %Y").to_string(),
                Err(_) => "Invalid date".to_string(),
            };

            let tech_stack_html = card.tech_stack.join(", ");

            card_template
                .replace("{title}", &card.title.replace("\"", ""))
                .replace("{date}", &formatted_date)
                .replace("{description}", &card.description.replace("\"", ""))
                .replace("{slug}", &card.slug)
                .replace("{image}", &card.image)
                .replace("{tech_stack}", &tech_stack_html)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let final_html = html_wrapper.replace("{}", &cards_html);
    HttpResponse::Ok().body(final_html)
}

pub async fn show_project(slug: web::Path<String>) -> impl Responder {
    // Stub implementation - returns a simple message for now
    HttpResponse::Ok().body(format!("Project '{}' coming soon!", slug))
}