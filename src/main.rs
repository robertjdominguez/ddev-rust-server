use actix_files as actix_fs;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
mod transformation;
use chrono::DateTime;

use transformation::{
    get_posts, order_cards, read_file_and_create_card, transform_markdown_to_html,
};

#[get("/")]
async fn get_index() -> impl Responder {
    let html = fs::read_to_string("templates/index.html");

    let response = match html {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(error) => HttpResponse::InternalServerError()
            .body(format!("Error getting the index template: {}", error)),
    };

    response
}

#[get("/posts")]
async fn posts() -> impl Responder {
    // Read the HTML wrapper template
    let html_wrapper = match fs::read_to_string("templates/posts.html") {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading HTML wrapper: {}", e))
        }
    };

    // Attempt to fetch posts
    let posts = match get_posts("posts".to_string()).await {
        Ok(posts) => posts,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching posts: {}", e))
        }
    };

    // Process each post into a card
    let mut cards = Vec::new();
    for post in posts {
        match read_file_and_create_card(&post).await {
            Ok(card) => cards.push(card),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error processing a post: {}", e))
            }
        }
    }

    // Read the card template (assuming it's static and doesn't change per card)
    let card_template = match fs::read_to_string("templates/card.html") {
        Ok(template) => template,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading card template: {}", e))
        }
    };

    let cards = order_cards(&cards).await;
    // Construct the HTML content for each card
    let cards_html: String = cards
        .iter()
        .map(|card| {
            // Attempt to parse the datetime string
            let formatted_date = match DateTime::parse_from_rfc3339(&card.created_at) {
                Ok(datetime) => datetime.format("%B %d, %Y %H:%M").to_string(),
                Err(_) => "Invalid date".to_string(), // Fallback in case of error
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
    // Insert the cards HTML into the HTML wrapper
    let final_html = html_wrapper.replace("{}", &cards_html);

    HttpResponse::Ok().body(final_html)
}

/**
* Here, we're creating an async function that will return the
* requested blog's content. We'll handle the md » html transformation
* in another module, but this is the route that will be called by the client.
*/
async fn show_post(slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();
    let html_content = transform_markdown_to_html(slug);
    HttpResponse::Ok().body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(posts)
            .service(get_index)
            .route("/posts/{slug}", web::get().to(show_post))
            .service(actix_fs::Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
