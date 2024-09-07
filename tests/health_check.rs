use forge::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{collections::HashMap, net::TcpListener};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuratin");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"create database "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health-check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn announce_returns_200_for_valid_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let mut body = HashMap::new();
    body.insert("name", "Angelie");
    body.insert("announcement", "Welcome to area 51");

    let response = client
        .post(format!("{}/announcement", test_app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select name, announcement from announcement",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved announcement");

    assert_eq!(saved.name, "Angelie");
    assert_eq!(saved.announcement, "Welcome to area 51");
}

#[tokio::test]
async fn announce_returns_400_data_missing() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (HashMap::from([("name", "Angelie")]), "Missing announcement"),
        (
            HashMap::from([("announcement", "Welcome to area 51")]),
            "Missing name",
        ),
        (HashMap::new(), "Missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/announcement", test_app.address))
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), "{}", error_message);
    }
}
