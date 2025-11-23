// tests/requests/auth_test.rs
use loco_rs::testing;
use serde_json::json;

#[tokio::test]
async fn test_register_endpoint() {
    let app = testing::app::test_app().await;
    
    let response = app
        .post("/api/auth/register")
        .json(&json!({
            "email": "newuser@example.com",
            "password": "password123",
            "name": "New User"
        }))
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_login_endpoint() {
    let app = testing::app::test_app().await;
    
    // まずユーザーを作成
    app.post("/api/auth/register")
        .json(&json!({
            "email": "logintest@example.com",
            "password": "password123",
            "name": "Login Test"
        }))
        .await;

    // ログインテスト
    let response = app
        .post("/api/auth/login")
        .json(&json!({
            "email": "logintest@example.com",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json();
    assert!(body["token"].is_string());
}

