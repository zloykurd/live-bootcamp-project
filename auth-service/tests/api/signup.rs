use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn sign_up_returns_200() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let response = app
        .post_sign_up(&serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": false
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

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
            "password": " ",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "1 ",
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
               "password": null,
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_sign_up(test_case).await;
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
    let random_email = get_random_email();
    let app = TestApp::new().await;
    let test_cases = [serde_json::json!({
        "email": random_email,
           "password": "password123",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_sign_up(test_case).await;
        assert_eq!(response.status().as_u16(), 201);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let random_email = get_random_email();
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": "testuser",
               "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "testuser@#test",
               "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
               "password": "passwo",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
               "password": "passwor",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_sign_up(test_case).await;
        assert_eq!(response.status(), 400);
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let random_email = get_random_email();
    let email = random_email.as_str();
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": email,
               "password": "password321",
            "requires2FA": true
        }),
        serde_json::json!({
          "email": email,
               "password": "password123",
            "requires2FA": false
        }),
    ];

    let mut created = false;
    for test_case in test_cases.iter() {
        let response = app.post_sign_up(test_case).await;
        if created {
            assert_eq!(response.status().as_u16(), 409);
        } else {
            created = response.status().as_u16() == 201;
        }
    }
}
