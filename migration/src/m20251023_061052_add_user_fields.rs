use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Postテーブル作成
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title).not_null())
                    .col(string(Post::Text).not_null()) // Textに変更（元はContent？）
                    .col(integer(Post::UserId).not_null())
                    .to_owned(),                    
            )
            .await?;
            
        // Userテーブルに列を追加
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(string_null(User::VerificationToken))
                    .add_column(string_null(User::ResetToken))
                    .add_column(string_null(User::Pid))
                    .add_column(string_null(User::MagicLinkToken)) // MagicToken → MagicLinkToken
                    .add_column(timestamp_null(User::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Postテーブル削除
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
            
        // Userテーブルから列を削除
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::VerificationToken)
                    .drop_column(User::ResetToken)
                    .drop_column(User::Pid)
                    .drop_column(User::MagicLinkToken) // MagicToken → MagicLinkToken
                    .drop_column(User::EmailVerifiedAt)
                    .to_owned(),
            )            
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text, // Content → Text に変更
    UserId,
}

#[derive(DeriveIden)] // DeriveIdenを追加
enum User {
    Table,
    Id,
    Email,
    Password,
    VerificationToken,
    ResetToken,
    Pid,
    MagicLinkToken, // MagicToken → MagicLinkToken に変更
    EmailVerifiedAt,
}
