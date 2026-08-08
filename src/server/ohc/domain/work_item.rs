use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "work_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub source: String,
    pub payload: Option<serde_json::Value>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::omnichannel_tables::Entity",
        from = "Column::CustomerId",
        to = "super::omnichannel_tables::Column::Id"
    )]
    CustomerProfile,
    #[sea_orm(has_many = "super::agent_draft::Entity")]
    AgentDraft,
}

impl Related<super::omnichannel_tables::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomerProfile.def()
    }
}

impl Related<super::agent_draft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AgentDraft.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
