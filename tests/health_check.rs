use std::net::TcpListener;

use rustmailer::run;

#[tokio::test]
async fn test_name() {
    let host_port = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/health_check");
    let res = client.get(url).send().await.expect("Failed to execute request.");
    assert!(res.status().is_success());
    assert_eq!(Some(0),res.content_length());
}

fn spawn_app() -> String{
    let tcplistner = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random address");
    let host_port = tcplistner.local_addr().expect("Failed to get the address").to_string();
    let server = run(tcplistner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    println!("{host_port}");
    host_port
}