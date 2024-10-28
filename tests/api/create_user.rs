use std::collections::HashMap;

use sqlx::{Connection, PgConnection};

use crate::helpers::{authenticate_user, spawn_app};

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
    // Arrange
    let test_app = spawn_app().await;
    let client = authenticate_user(&test_app.address).await;

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

    let saved =
        sqlx::query!("select display_name, phone_number from raabta_user order by created_at")
            .fetch_all(&mut connection)
            .await
            .expect("Failed to fetch newly inserted user");

    // Assert
    assert_eq!(200, response.status().as_u16());

    assert_eq!(saved.len(), 2);
    assert_eq!(
        saved.first().unwrap().display_name,
        body.get("display_name").unwrap().to_string()
    );
    assert_eq!(
        saved.first().unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
    assert_eq!(
        saved.last().unwrap().display_name,
        body.get("display_name").unwrap().to_string() + "'s Parent"
    );
    assert_eq!(
        saved.last().unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
}
// TODO: add tests for empty phone number

#[tokio::test]
async fn create_user_returns_400_data_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let client = authenticate_user(&test_app.address).await;

    let test_cases = vec![
        (
            HashMap::from([("display_name", "Angelie")]),
            "Missing phone_number",
        ),
        (
            HashMap::from([("phone_number", "0333-3787822")]),
            "Missing display name",
        ),
        (HashMap::new(), "Missing both"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/user", test_app.address))
            .form(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16(), "{}", error_message);
    }
}

#[tokio::test]
async fn create_user_returns_200_with_err_msg_when_fields_are_present_but_empty() {
    let test_app = spawn_app().await;
    let client = authenticate_user(&test_app.address).await;
    let test_cases = vec![
        (
            HashMap::from([("display_name", ""), ("phone_number", "0333-3888866")]),
            "Display name is empty",
        ),
        (
            HashMap::from([("display_name", ""), ("phone_number", "")]),
            "Display name is empty",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/user", test_app.address))
            .form(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16(), "{}", error_message);
        assert!(response.text().await.unwrap().contains(error_message));
    }
}
