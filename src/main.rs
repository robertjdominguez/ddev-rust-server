use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
mod transformation;

use transformation::transform_markdown_to_html;

#[get("/posts")]
async fn posts() -> impl Responder {
    HttpResponse::Ok().body("Posts")
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
            .route("/posts/{slug}", web::get().to(show_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
