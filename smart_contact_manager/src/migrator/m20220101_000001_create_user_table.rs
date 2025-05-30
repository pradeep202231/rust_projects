use sea_orm_migration::{prelude::*, schema::*, seaql_migrations::Column};

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
                        ColumnDef::new(User::Username)
                            .string() // Specify the column type first
                            .not_null() // Add NOT NULL constraint
                            .unique_key(), // Add UNIQUE constraint
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string() // Specify the column type first
                            .not_null() // Add NOT NULL constraint
                            .unique_key(), // Add UNIQUE constraint
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string() // Specify the column type first
                            // .not_null(), // Add NOT NULL constraint
                    )
                    .col(
                        ColumnDef::new(User::Provider)
                            .string() // Specify the column type first
                            .not_null() // Add NOT NULL constraint
                            // .unique_key(), // Add UNIQUE constraint
                    )
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
pub enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    Provider
}
