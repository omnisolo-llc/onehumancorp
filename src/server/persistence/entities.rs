use sea_orm::entity::prelude::*;

pub mod schema_version {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "onehumancorp_schema_versions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub applied_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod user {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub username: String,
        pub email: String,
        pub password_hash: String,
        pub roles: Json,
        pub active: bool,
        pub tenant_id: String,
        pub oidc_subject: Option<String>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod product {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "products")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub title: String,
        #[sea_orm(column_type = "Text", nullable)]
        pub description: Option<String>,
        #[sea_orm(column_name = "type")]
        pub item_type: Option<String>,
        pub price_cents: Option<i64>,
        pub inventory_count: Option<i32>,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}


pub mod chat_inboxes {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chat_inboxes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub name: String,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_channels {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chat_channels")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub inbox_id: String,
        pub channel_type: String,
        pub config: Option<Json>,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_contacts {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chat_contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub name: Option<String>,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_conversations {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chat_conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub inbox_id: String,
        pub contact_id: String,
        pub assignee_id: Option<String>,
        pub status: String,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_messages {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chat_messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub tenant_id: String,
        pub conversation_id: String,
        pub sender_type: String,
        pub sender_id: Option<String>,
        pub content: String,
        pub created_at: Option<DateTimeUtc>,
        pub updated_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
