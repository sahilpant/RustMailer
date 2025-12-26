use std::net::TcpListener;

use rustmailer::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", config.application_host, config.application_port);
    let tcplistner = TcpListener::bind(address).expect("Unable to bind to port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    println!("Server running at port 8080");
    run(tcplistner, connection_pool)?.await
}
