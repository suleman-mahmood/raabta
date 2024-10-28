use std::collections::HashMap;

use regex::Regex;

use crate::helpers::spawn_app;

#[tokio::test]
async fn submit_login_correct_credentials() {
    // Arrange
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let test_app = spawn_app().await;

    let mut body = HashMap::new();
    body.insert("email", "admin@raabta.com");
    body.insert("password", "root");

    // Act
    let response = client
        .post(format!("{}/submit-login", test_app.address))
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    for c in response.cookies() {
        assert_eq!(c.name(), "token");
        assert!(
            Regex::new(r#"^([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_\-\+\/=]*)"#)
                .unwrap()
                .is_match(c.value())
        );
        assert!(c.secure());
        assert!(c.http_only());
        assert_eq!(c.path(), Some("/"));
        assert!(c.same_site_strict());
    }
}

#[tokio::test]
async fn submit_login_wrong_credentials() {
    // Arrange
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let test_app = spawn_app().await;

    let mut body = HashMap::new();
    body.insert("email", "vader@raabta.com");
    body.insert("password", "doesnt_know_pass");

    // Act
    let response = client
        .post(format!("{}/submit-login", test_app.address))
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert!(response.cookies().next().is_none());

    assert!(response.text().await.unwrap().contains("Wrong credentials"));
}
