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
