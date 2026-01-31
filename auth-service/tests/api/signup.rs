use crate::helpers::TestApp;

#[tokio::test]
async fn sign_up_returns_200() {
    let app = TestApp::new().await;
    let response = app.get_sign_up().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}