use crate::helpers::TestApp;

#[tokio::test]
pub async fn health_check_should_return_200() {
    let client = TestApp::new().await;
    let response = client.get_health().await;
    assert_eq!(response.status().as_u16(), 200);
}
