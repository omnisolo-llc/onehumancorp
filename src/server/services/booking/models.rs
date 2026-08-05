use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod calendars {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "calendars")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub name: String,
        pub display_timezone: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::time_slots::Entity")]
        TimeSlot,
    }

    impl Related<super::time_slots::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::TimeSlot.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod time_slots {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "time_slots")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub calendar_id: String,
        pub service_id: Option<String>,
        pub start_time: DateTime<Utc>,
        pub end_time: DateTime<Utc>,
        pub status: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::calendars::Entity",
            from = "Column::CalendarId",
            to = "super::calendars::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Calendar,
    }

    impl Related<super::calendars::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Calendar.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod bookings {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "bookings")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub customer_id: String,
        pub service_id: Option<String>,
        pub payment_intent_id: Option<String>,
        pub start_time: DateTime<Utc>,
        pub end_time: Option<DateTime<Utc>>,
        pub status: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
