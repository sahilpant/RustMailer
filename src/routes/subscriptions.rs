use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
/// Actix web uses a type-map to represnt its application state: 
/// a Hashmap that stores arbitary data (using the Any type) 
/// against their unique type identifier (obtained by TypeId::of)
pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
    )
    .execute(connection.get_ref())
    .await 
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
