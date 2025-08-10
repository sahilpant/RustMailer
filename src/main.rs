use std::net::TcpListener;

use rustmailer::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let host_port = format!("{}:{}",std::env::var("SERVER_HOST").unwrap_or("0.0.0.0".into()),std::env::var("SERVER_PORT").unwrap_or("8080".into()));
    let tcplistner = TcpListener::bind(host_port).expect("Unable to bind to port");
    println!("Server running at port 8080");
    run(tcplistner)?.await
}
