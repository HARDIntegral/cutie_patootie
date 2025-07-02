use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hi babe 💖 from backend!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
