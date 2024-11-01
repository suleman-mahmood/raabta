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
    let test_cases = vec![
        (
            HashMap::from([
                ("display_name", "Luke Skywalker"),
                ("phone_number", "0333-3452599"),
                ("radio_user_type", "student-parent"),
            ]),
            "valid data",
        ),
        (
            HashMap::from([
                ("display_name", "Luke Skywalker"),
                ("phone_number", ""),
                ("radio_user_type", "student-parent"),
            ]),
            "Phone number is absent",
        ),
    ];

    // Act
    for (body, error_message) in &test_cases {
        let response = client
            .post(format!("{}/user", test_app.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(200, response.status().as_u16(), "{}", error_message);
    }
    let saved =
        sqlx::query!("select display_name, phone_number from raabta_user order by created_at")
            .fetch_all(&mut connection)
            .await
            .expect("Failed to fetch newly inserted user");
    assert_eq!(saved.len(), 4, "Fetched rows don't match expected count");
    let mut rows_iter = saved.iter();

    for (body, error_message) in test_cases {
        let curr_row = rows_iter.next();
        assert_eq!(
            curr_row.unwrap().display_name,
            body.get("display_name").unwrap().to_string(),
            "{}",
            error_message,
        );

        assert_eq!(
            curr_row.unwrap().phone_number,
            if body.get("phone_number").unwrap().is_empty() {
                None
            } else {
                Some(body.get("phone_number").unwrap().to_string())
            },
            "{}",
            error_message,
        );
        let curr_row = rows_iter.next();
        assert_eq!(
            curr_row.unwrap().display_name,
            body.get("display_name").unwrap().to_string() + "'s Parent",
            "{}",
            error_message,
        );
        assert_eq!(
            curr_row.unwrap().phone_number,
            if body.get("phone_number").unwrap().is_empty() {
                None
            } else {
                Some(body.get("phone_number").unwrap().to_string())
            },
            "{}",
            error_message,
        );
    }
}

#[tokio::test]
async fn create_user_common_email() {
    // Arrange
    let test_app = spawn_app().await;
    let client = authenticate_user(&test_app.address).await;

    let mut connection = PgConnection::connect_with(&test_app.config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let mut body = HashMap::new();
    body.insert("display_name", "Luke Skywalker");
    body.insert("phone_number", "0333-3452599");
    body.insert("radio_user_type", "student-parent");

    // Act
    let response = client
        .post(format!("{}/user", test_app.address))
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let response = client
        .post(format!("{}/user", test_app.address))
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "select display_name, phone_number, email from raabta_user order by created_at"
    )
    .fetch_all(&mut connection)
    .await
    .expect("Failed to fetch newly inserted user");

    // Assert
    assert_eq!(saved.len(), 4);

    assert_eq!(
        saved.first().unwrap().display_name,
        body.get("display_name").unwrap().to_string()
    );
    assert_eq!(
        saved.first().unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
    assert_eq!(saved.first().unwrap().email, "luke@riveroaks.com");

    assert_eq!(
        saved.get(1).unwrap().display_name,
        body.get("display_name").unwrap().to_string() + "'s Parent"
    );
    assert_eq!(
        saved.get(1).unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
    assert_eq!(saved.get(1).unwrap().email, "luke.parent@riveroaks.com");

    assert_eq!(
        saved.get(2).unwrap().display_name,
        body.get("display_name").unwrap().to_string()
    );
    assert_eq!(
        saved.get(2).unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
    assert_eq!(saved.get(2).unwrap().email, "luke1@riveroaks.com");

    assert_eq!(
        saved.last().unwrap().display_name,
        body.get("display_name").unwrap().to_string() + "'s Parent"
    );
    assert_eq!(
        saved.last().unwrap().phone_number,
        Some(body.get("phone_number").unwrap().to_string())
    );
    assert_eq!(saved.last().unwrap().email, "luke1.parent@riveroaks.com");
}

#[tokio::test]
async fn create_user_returns_400_data_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let client = authenticate_user(&test_app.address).await;

    let test_cases = vec![
        (
            HashMap::from([
                ("display_name", "Angelie"),
                ("radio_user_type", "student-parent"),
            ]),
            "Missing phone_number",
        ),
        (
            HashMap::from([
                ("phone_number", "0333-3787822"),
                ("radio_user_type", "student-parent"),
            ]),
            "Missing display name",
        ),
        (
            HashMap::from([
                ("display_name", "Angelie"),
                ("phone_number", "0333-3787822"),
            ]),
            "Missing radio user type",
        ),
        (HashMap::new(), "Missing all"),
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
            HashMap::from([
                ("display_name", ""),
                ("phone_number", "0333-3888866"),
                ("radio_user_type", "student-parent"),
            ]),
            "Display name is empty",
        ),
        (
            HashMap::from([
                ("display_name", ""),
                ("phone_number", ""),
                ("radio_user_type", "student-parent"),
            ]),
            "Display name is empty",
        ),
        (
            HashMap::from([
                ("display_name", ""),
                ("phone_number", ""),
                ("radio_user_type", ""),
            ]),
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
