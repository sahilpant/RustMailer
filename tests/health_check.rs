use std::net::TcpListener;

use rustmailer::run;

#[tokio::test]
async fn test_name() {
    let host_port = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/health_check");
    let res = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

async fn subscribe_returns_a_200_for_valid_form_data() {
    let host_port = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/subscriptions");

    let body = "name=sahil%20pant&email=sahilpant16%40gmail.com";
    let response = client
        .post(url)
        .header("Conten-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to get a response");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let host_port = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/subscriptions");
    let test_cases = vec![
        ("name=sahil%20pant", "missingtheemail"),
        ("email=sahilpant16n%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failedtoexecuterequest.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the pay load was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let tcplistner = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random address");
    let host_port = tcplistner
        .local_addr()
        .expect("Failed to get the address")
        .to_string();
    let server = run(tcplistner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    println!("{host_port}");
    host_port
}
