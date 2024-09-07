use forge::startup::run;
use std::{collections::HashMap, net::TcpListener};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn announce_returns_200_for_valid_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("name", "Angelie");
    body.insert("announcement", "Welcome to area 51");

    let response = client
        .post(format!("{}/announcement", app_address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn announce_returns_400_data_missing() {
    let app_address = spawn_app();
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
            .post(format!("{}/announcement", app_address))
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), "{}", error_message);
    }
}
