use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    email: String
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}