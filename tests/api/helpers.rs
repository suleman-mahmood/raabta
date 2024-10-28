use forge::{
    configuration::{get_configuration, DatabaseSettings, Settings},
    startup::run,
};
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{collections::HashMap, net::TcpListener};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub config: Settings,
}

pub async fn spawn_app() -> TestApp {
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
        config: configuration,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"create database "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn authenticate_user(test_app_address: &str) -> Client {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let mut body = HashMap::new();
    body.insert("email", "admin@raabta.com");
    body.insert("password", "root");

    client
        .post(format!("{}/submit-login", test_app_address))
        .form(&body)
        .send()
        .await
        .unwrap();

    client
}
