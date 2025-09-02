use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email": &random_email,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = TestApp::post_login(&app, test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message. 
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "",
            "password": "password123",
        }),
        serde_json::json!({
            "email": &random_email,
            "password": "",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = TestApp::post_login(&app, test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
        let error_response: ErrorResponse = response.json().await.unwrap();
        assert_eq!(error_response.error, "Invalid credentials");
    }
}