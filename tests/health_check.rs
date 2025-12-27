use rustmailer::{
    configuration::{DatabaseSettings, get_configuration},
    startup::run,
};
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let host_port = spawn_app().await.address;
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let host_port = app.address;
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/subscriptions");
    let connection = app.db_pool;

    let body = "name=sahil%20pant&email=sahilpant16%40gmail.com";
    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to get a response");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name From subscriptions",)
        .fetch_one(&connection)
        .await
        .expect("Failed to fetch saved data");

    assert_eq!(saved.email, "sahilpant16@gmail.com");
    assert_eq!(saved.name, "sahil pant");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let host_port = spawn_app().await.address;
    let client = reqwest::Client::new();
    let url = format!("http://{host_port}/subscriptions");
    let test_cases = vec![
        ("username=sahil%20pant", "missingtheemail"),
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

async fn spawn_app() -> TestApp {
    let tcplistner = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random address");
    let address = tcplistner
        .local_addr()
        .expect("Failed to get the address")
        .to_string();
    let mut configuation = get_configuration().expect("failed to read configuration");
    configuation.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(configuation.database).await;
    let server = run(tcplistner, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    println!("{address}");
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Unableto connect to db");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Unable to create the table");
    let connect_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Unable to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connect_pool)
        .await
        .expect("Failed to migrate the database");
    connect_pool
}
