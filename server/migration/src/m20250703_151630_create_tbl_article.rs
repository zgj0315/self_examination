use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TblArticle::Table)
                    .if_not_exists()
                    .col(pk_auto(TblArticle::Id))
                    .col(string(TblArticle::Title))
                    .col(text(TblArticle::Content))
                    .col(date_time(TblArticle::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(TblArticle::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TblArticle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TblArticle {
    Table,
    Id,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
}
