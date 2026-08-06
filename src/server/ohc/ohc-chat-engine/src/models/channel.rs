use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "channel_adapters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub channel_id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub provider_type: String,
    pub config: serde_json::Value,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::inbox::Entity",
        from = "Column::InboxId",
        to = "super::inbox::Column::InboxId"
    )]
    Inbox,
}

impl Related<super::inbox::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inbox.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
