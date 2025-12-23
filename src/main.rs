use std::net::TcpListener;

use rustmailer::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}",config.application_host,config.application_port);
    let tcplistner = TcpListener::bind(address).expect("Unable to bind to port");
    println!("Server running at port 8080");
    run(tcplistner)?.await
}
