use rustmailer::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use tracing_log::LogTracer;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirects all 'log's' events to subscriber
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("rustmailer".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    
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
