pub mod constants;
pub mod auth;

pub use constants::{JWT_COOKIE_NAME, JWT_SECRET};
pub use auth::{generate_auth_cookie, validate_token};