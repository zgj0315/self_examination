//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_pdf_article")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub pdf_content: Vec<u8>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_pdf_article_access_log::Entity")]
    TblPdfArticleAccessLog,
}

impl Related<super::tbl_pdf_article_access_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblPdfArticleAccessLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
