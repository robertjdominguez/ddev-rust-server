use actix_files as actix_fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::fs;

mod handlers;
mod models;
mod services;
mod utils;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(posts)
            .service(projects)
            .service(get_index)
            .route("/posts/{slug}", web::get().to(show_post))
            .route("/projects/{slug}", web::get().to(show_project))
            .service(actix_fs::Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
