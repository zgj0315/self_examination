use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TblPdfArticle::Table)
                    .if_not_exists()
                    .col(pk_auto(TblPdfArticle::Id))
                    .col(string(TblPdfArticle::Title))
                    .col(binary(TblPdfArticle::PdfContent))
                    .col(date_time(TblPdfArticle::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(TblPdfArticle::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TblPdfArticle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TblPdfArticle {
    Table,
    Id,
    Title,
    PdfContent,
    CreatedAt,
    UpdatedAt,
}
