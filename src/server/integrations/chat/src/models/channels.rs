use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_channels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<Json>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::inboxes::Entity",
        from = "Column::InboxId",
        to = "super::inboxes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Inbox,
}

impl Related<super::inboxes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inbox.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
