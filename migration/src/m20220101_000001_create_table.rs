use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Friends::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Friends::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Friends::Name).string().not_null())
                    .col(ColumnDef::new(Friends::Pronouns).string().not_null())
                    .col(ColumnDef::new(Friends::Notes).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Friends::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Friends {
    Table,
    Id,
    Name,
    Pronouns,
    Notes,
}
