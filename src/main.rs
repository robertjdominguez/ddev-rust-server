use actix_files as actix_fs;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use log::debug;
use std::fs;
mod transformation;

use transformation::{get_posts, read_file_and_create_post, transform_markdown_to_html};

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
    let html_wrapper = fs::read_to_string("templates/posts.html");

    match get_posts("posts".to_string()).await {
        Ok(posts) => {
            for file in &posts {
                // At this point, we have the filenames and can generate a card for each
                let frontmatter = read_file_and_create_post(file).await;
            }
            HttpResponse::Ok().body("Placeholder response with posts data")
        }
        Err(e) => {
            log::error!("Failed to get posts: {}", e);
            HttpResponse::InternalServerError().body("Error retrieving posts")
        }
    }
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
