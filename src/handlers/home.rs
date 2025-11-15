use actix_web::{get, HttpResponse, Responder};
use std::fs;

#[get("/")]
pub async fn get_index() -> impl Responder {
    let html = fs::read_to_string("templates/index.html");

    let response = match html {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(error) => HttpResponse::InternalServerError()
            .body(format!("Error getting the index template: {}", error)),
    };

    response
}