#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

/// deserialize `Info` from request's body
fn index(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.name)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(web::resource("/{id}").route(web::post().to(index))))
        .bind("127.0.0.1:8080")?
        .run()
}
