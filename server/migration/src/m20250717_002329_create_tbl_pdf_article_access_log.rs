use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250716_151156_create_tbl_pdf_article::TblPdfArticle;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TblPdfArticleAccessLog::Table)
                    .if_not_exists()
                    .col(pk_auto(TblPdfArticleAccessLog::Id))
                    .col(integer(TblPdfArticleAccessLog::PdfArticleId))
                    .col(string(TblPdfArticleAccessLog::SrcIp))
                    .col(string(TblPdfArticleAccessLog::UserAgent))
                    .col(
                        date_time(TblPdfArticleAccessLog::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                TblPdfArticleAccessLog::Table,
                                TblPdfArticleAccessLog::PdfArticleId,
                            )
                            .to(TblPdfArticle::Table, TblPdfArticle::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TblPdfArticleAccessLog::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TblPdfArticleAccessLog {
    Table,
    Id,
    PdfArticleId,
    SrcIp,
    UserAgent,
    CreatedAt,
}
