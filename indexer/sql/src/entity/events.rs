//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Default,
    serde :: Serialize,
    serde :: Deserialize,
)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub message_id: Option<Uuid>,
    pub created_at: DateTime,
    pub r#type: String,
    pub method: Option<String>,
    pub event_status: i16,
    pub commitment_status: i16,
    pub transaction_type: i16,
    pub transaction_idx: i32,
    pub message_idx: Option<i32>,
    pub event_idx: i32,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,
    pub block_height: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
