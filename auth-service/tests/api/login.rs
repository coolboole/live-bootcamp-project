use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email,
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};

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

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let correct_password = "password123";
    let wrong_password = "wrongpassword";

    // First, create a new account
    let signup_body = serde_json::json!({
        "email": &random_email,
        "password": correct_password,
        "requires2FA": true
    });
    
    let signup_response = TestApp::post_signup(&app, &signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201, "Failed to create test account");

    // Now try to login with the correct email but wrong password
    let login_body = serde_json::json!({
        "email": &random_email,
        "password": wrong_password
    });
    
    let login_response = TestApp::post_login(&app, &login_body).await;
    assert_eq!(login_response.status().as_u16(), 401, "Expected 401 for incorrect password");
    
    let error_response: ErrorResponse = login_response.json().await.unwrap();
    assert_eq!(error_response.error, "Invalid credentials");
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let two_fa_code_store = app.two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&Email::parse(random_email).unwrap())
        .await
        .expect("Failed to get 2FA code");

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);
}