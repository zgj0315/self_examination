use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TblFile::Table)
                    .if_not_exists()
                    .col(pk_auto(TblFile::Id))
                    .col(binary(TblFile::Content))
                    .col(date_time(TblFile::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TblFile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TblFile {
    Table,
    Id,
    Content,
    CreatedAt,
}
