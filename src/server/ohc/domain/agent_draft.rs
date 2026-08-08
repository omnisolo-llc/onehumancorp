use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "agent_draft")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub work_item_id: Uuid,
    pub response: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::work_item::Entity",
        from = "Column::WorkItemId",
        to = "super::work_item::Column::Id"
    )]
    WorkItem,
}

impl Related<super::work_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
