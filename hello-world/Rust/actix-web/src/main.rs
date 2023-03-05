use actix_web::{App, get, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
