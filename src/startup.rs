use std::net::TcpListener;
use actix_web::{App, HttpServer, dev::Server, web, middleware::Logger};
use env_logger::Env;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(
    listner:TcpListener,
    db_pool: PgPool
) -> Result<Server,std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}