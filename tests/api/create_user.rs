use crate::server_setup::spawn_app;

// #[tokio::test]
// async fn announce_returns_200_for_valid_data() {
//     let test_app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let mut connection = PgConnection::connect_with(&test_app.config.database.with_db())
//         .await
//         .expect("Failed to connect to Postgres.");
//
//     let mut body = HashMap::new();
//     body.insert("name", "Angelie");
//     body.insert("announcement", "Welcome to area 51");
//
//     let response = client
//         .post(format!("{}/announcement", test_app.address))
//         .json(&body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//
//     assert_eq!(200, response.status().as_u16());
//
//     let saved = sqlx::query!("select name, announcement from announcement",)
//         .fetch_one(&mut connection)
//         .await
//         .expect("Failed to fetch saved announcement");
//
//     assert_eq!(saved.name, "Angelie");
//     assert_eq!(saved.announcement, "Welcome to area 51");
// }
//
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
