use std::collections::HashMap;

use sqlx::{Connection, PgConnection};

use crate::server_setup::spawn_app;

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
    // Setup
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let mut connection = PgConnection::connect_with(&test_app.config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let mut body = HashMap::new();
    body.insert("display_name", "Luke Skywalker");
    body.insert("phone_number", "0333-3452599");

    // Act
    let response = client
        .post(format!("{}/user", test_app.address))
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select display_name, phone_number from raabta_user",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch newly inserted user");

    assert_eq!(
        saved.display_name,
        body.get("display_name").unwrap().to_string()
    );
    assert_eq!(
        saved.phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
}

// #[tokio::test]
// async fn announce_returns_400_data_missing() {
//     let test_app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         (HashMap::from([("name", "Angelie")]), "Missing announcement"),
//         (
//             HashMap::from([("announcement", "Welcome to area 51")]),
//             "Missing name",
//         ),
//         (HashMap::new(), "Missing both"),
//     ];
//
//     for (invalid_body, error_message) in test_cases {
//         let response = client
//             .post(format!("{}/announcement", test_app.address))
//             .json(&invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request.");
//
//         assert_eq!(400, response.status().as_u16(), "{}", error_message);
//     }
// }
// #[tokio::test]
// async fn announce_returns_400_when_fields_are_present_but_empty() {
//     let test_app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         (
//             HashMap::from([("name", ""), ("announcement", "Welcome to area 51")]),
//             "name present but empty",
//         ),
//         (
//             HashMap::from([("name", "Suleman"), ("announcement", "")]),
//             "announcement present but empty",
//         ),
//         (
//             HashMap::from([("name", ""), ("announcement", "")]),
//             "both present but empty",
//         ),
//     ];
//
//     for (invalid_body, error_message) in test_cases {
//         let response = client
//             .post(format!("{}/announcement", test_app.address))
//             .json(&invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request.");
//
//         assert_eq!(400, response.status().as_u16(), "{}", error_message);
//     }
// }
