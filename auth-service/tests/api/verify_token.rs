use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&serde_json::json!({})).await;
    assert_eq!(response.status().as_u16(), 422);
}
