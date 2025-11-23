// tests/models/user_test.rs
use blogapp05::models::{
    user::{ActiveModel, Model, RegisterParams},
};
use loco_rs::testing;

#[tokio::test]
async fn integration_test_user_registration() {
    let db = testing::db::test_db().await;
    
    let params = RegisterParams {
        email: "integration@example.com".to_string(),
        password: "password123".to_string(),
        name: "Integration User".to_string(),
    };

    let user = ActiveModel::create_with_password(&db, &params)
        .await
        .expect("Failed to create user");

    assert_eq!(user.email, "integration@example.com");
    assert_ne!(user.password, "password123"); // ハッシュ化されているべき
}

