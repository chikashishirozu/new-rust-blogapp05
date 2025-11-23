// tests/common/mod.rs
use blogapp05::models::user::{ActiveModel, RegisterParams};
use loco_rs::testing;
use sea_orm::DatabaseConnection;

pub async fn create_test_user(db: &DatabaseConnection, email: &str) -> blogapp05::models::_entities::user::Model {
    let params = RegisterParams {
        email: email.to_string(),
        password: "testpassword".to_string(),
        name: "Test User".to_string(),
    };

    ActiveModel::create_with_password(db, &params)
        .await
