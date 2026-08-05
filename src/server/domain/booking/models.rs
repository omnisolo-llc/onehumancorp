use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "booking_slots")]
pub struct BookingSlotModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub tenant_id: String,
    pub service_id: Option<String>,
    pub resource_id: Option<String>,
    pub start_time: DateTimeUtc,
    pub end_time: DateTimeUtc,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum BookingSlotRelation {}

impl ActiveModelBehavior for BookingSlotActiveModel {}
