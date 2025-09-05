use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::{cookie::CookieStore, Url};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    let response_body: ErrorResponse = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");

    assert_eq!(response_body.error, "Missing token");
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = "BAD_Pa$$w0rd!";

    // register user
    app.post_signup(
        &serde_json::json!({
            "email": &email,
            "password": password,
            "requires2FA": false
        }),
    ).await;

    // login user
    app.post_login(
        &serde_json::json!({
            "email": &email,
            "password": password
        }),
    ).await;

    // ensure cookie is set
    assert!(app.cookie_jar.cookies(&Url::parse(&app.address).unwrap()).is_some());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = "BAD_Pa$$w0rd!";

    // register user
    app.post_signup(
        &serde_json::json!({
            "email": &email,
            "password": password,
            "requires2FA": false
        }),
    ).await;

    // login user
    app.post_login(
        &serde_json::json!({
            "email": &email,
            "password": password
        }),
    ).await;

    // ensure cookie is set
    assert!(app.cookie_jar.cookies(&Url::parse(&app.address).unwrap()).is_some());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
    let response_body: ErrorResponse = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");
    assert_eq!(response_body.error, "Missing token");
}