use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Pid)
                            .string()  // UUIDを文字列として保存
                            .not_null()
                            .unique_key()  // ユニーク制約を追加
                    )
                    // ... 他のカラムは同じ
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::ApiKey).string().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::ResetToken).string().null())
                    .col(ColumnDef::new(User::ResetSentAt).timestamp().null())
                    .col(ColumnDef::new(User::EmailVerificationToken).string().null())
                    .col(ColumnDef::new(User::EmailVerificationSentAt).timestamp().null())
                    .col(ColumnDef::new(User::EmailVerifiedAt).timestamp().null())
                    .col(ColumnDef::new(User::MagicLinkToken).string().null())
                    .col(ColumnDef::new(User::MagicLinkExpiration).timestamp().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // ユニークインデックスの作成
        manager
            .create_index(
                Index::create()
                    .name("idx_user_pid_unique")
                    .table(User::Table)
                    .col(User::Pid)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_email_unique")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_api_key_unique")
                    .table(User::Table)
                    .col(User::ApiKey)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Pid,
    Email,
    Password,
    ApiKey,
    Name,
    ResetToken,
    ResetSentAt,
    EmailVerificationToken,
    EmailVerificationSentAt,
    EmailVerifiedAt,
    MagicLinkToken,
    MagicLinkExpiration,
    CreatedAt,
    UpdatedAt,
}
