use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::SignupResponse, ErrorResponse};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "password": "PASSWORD456"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = TestApp::post_signup(&app, test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let response = TestApp::post_signup(
        &app, 
        &serde_json::json!({
            "email": &random_email,
            "password": "password123",
            "requires2FA": true
        }),
    ).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

/// The signup route should return a 400 HTTP status code if an invalid input is sent.
/// The input is considered invalid if:
/// - The email is empty or does not contain '@'
/// - The password is less than 8 characters
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "testatexample.com",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "test@example.com",
            "password": "a",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let request_body = serde_json::json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response1 = TestApp::post_signup(&app, &request_body).await;
    assert_eq!(response1.status().as_u16(), 201);
    let response2 = TestApp::post_signup(&app, &request_body).await;
    assert_eq!(response2.status().as_u16(), 409);

    assert_eq!(
        response2
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
