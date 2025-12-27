use rustmailer::{configuration::get_configuration, startup::run, telemetry::{get_subscriber, init_subscriber}};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(get_subscriber(std::io::stdout));
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", config.application_host, config.application_port);
    let tcplistner = TcpListener::bind(address).expect("Unable to bind to port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    println!("Server running at port 8080\n");
    run(tcplistner, connection_pool)?.await
}
