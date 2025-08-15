use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct FormData {
    username: String,
    email: String
}

async  fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listner:TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::get().to(subscribe))
    })
    .listen(listner)?
    .run();
    Ok(server)
}