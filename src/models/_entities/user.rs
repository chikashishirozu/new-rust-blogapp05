use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub password: String,
    pub verification_token: Option<String>,
    pub reset_token: Option<String>,
    pub pid: Option<String>,
    pub magic_link_token: Option<String>, // magic_token → magic_link_token
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub api_key: String,
    pub name: String,
    pub reset_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,   
    pub magic_link_expiration: Option<DateTimeWithTimeZone>, // 追加
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

// impl ActiveModelBehavior for ActiveModel {}

// メソッド実装を修正
impl Model {
    pub async fn create_with_password(
        db: &DbConn, 
        params: crate::models::user::RegisterParams
    ) -> Result<Self, DbErr> {
        use bcrypt::{hash, DEFAULT_COST};
        
        let hashed_password = hash(&params.password, DEFAULT_COST)
            .map_err(|_| DbErr::Custom("Password hashing failed".to_string()))?;
            
        let user = ActiveModel {
            email: Set(params.email),
            password: Set(hashed_password),
            name: Set(params.name),
            pid: Set(Some(Uuid::new_v4().to_string())),
            ..Default::default()
        };
        
        user.insert(db).await
    }

    // メソッドシグネチャを修正（ActiveModelではなく&mut selfを使用）
    pub async fn set_email_verification_sent(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.verification_token = Set(Some(Uuid::new_v4().to_string()));
        active_model.update(db).await
    }
    
    pub async fn find_by_verification_token(db: &DbConn, token: &str) -> Result<Option<Self>, sea_orm::DbErr> {
        Entity::find().filter(Column::VerificationToken.eq(token)).one(db).await
    }

    pub async fn find_by_reset_token(db: &DbConn, token: &str) -> Result<Option<Self>, sea_orm::DbErr> {
        Entity::find().filter(Column::ResetToken.eq(token)).one(db).await
    }

    pub async fn find_by_pid(db: &DbConn, pid: &str) -> Result<Option<Self>, sea_orm::DbErr> {
        Entity::find().filter(Column::Pid.eq(pid)).one(db).await
    }
    
    pub async fn find_by_magic_token(db: &DbConn, token: &str) -> Result<Option<Self>, DbErr> {
        Entity::find().filter(Column::MagicToken.eq(token)).one(db).await
    }    
    
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }

    pub fn generate_jwt(&self, secret: &str, expiration: i64) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: self.pid.as_ref().map(|p| p.to_string()).unwrap_or_default(),
            exp: (Utc::now() + Duration::seconds(expiration)).timestamp() as usize,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub async fn set_email_verification_sent(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        self.verification_token = Set(Some(Uuid::new_v4().to_string()));
        self.update(db).await
    }

    pub async fn set_forgot_password_sent(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        self.reset_token = Set(Some(Uuid::new_v4().to_string()));
        self.update(db).await
    }
    
    pub async fn reset_password(&mut self, db: &DbConn, password: &str) -> Result<Self, DbErr> {
        let hashed_password = hash(password, DEFAULT_COST).map_err(|_| DbErr::Custom("Password hashing failed".to_string()))?;
        self.password = Set(hashed_password);
        self.reset_token = Set(None);
        self.update(db).await
    }

    pub async fn create_magic_link(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        self.magic_token = Set(Some(Uuid::new_v4().to_string()));
        self.update(db).await
    }

    pub async fn clear_magic_link(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        self.magic_token = Set(None);
        self.email_verified_at = Set(Some(Utc::now().fixed_offset()));
        self.update(db).await
    }

    pub async fn verified(&mut self, db: &DbConn) -> Result<Self, DbErr> {
        self.email_verified_at = Set(Some(Utc::now().fixed_offset()));
        self.verification_token = Set(None);
        self.update(db).await
    }    
}

// JWTクレーム用構造体
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}    
