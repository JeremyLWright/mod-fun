use actix_web::{web, App, HttpServer, Responder, HttpResponse};

fn index(info: web::Path<(u32, String)>) -> impl Responder {

    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    let app = App::new().service(
            web::resource("/{id}")
            .route(web::post().to(move || HttpResponse::Ok()))
        );

    HttpServer::new(
        move ||  app
    )
        .bind("127.0.0.1:8080")?
        .run()

}