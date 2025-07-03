use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TblLog::Table)
                    .if_not_exists()
                    .col(pk_auto(TblLog::Id))
                    .col(text(TblLog::Content))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TblLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TblLog {
    Table,
    Id,
    Content,
}
