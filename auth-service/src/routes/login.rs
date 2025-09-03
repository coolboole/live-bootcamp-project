use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User, UserStoreError, email::Email, password::Password},
};

pub async fn login(
    state: State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = 
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = 
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, false);

    let user_store = state.user_store.read().await;

    user_store.validate_user(&user.email, &user.password).await.map_err(|e| match e {
        UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
        UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
        _ => AuthAPIError::UnexpectedError,
    })?;

    let response = Json(LoginResponse {
        message: "Login successful!".to_string(),
    });

    Ok((StatusCode::OK, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}