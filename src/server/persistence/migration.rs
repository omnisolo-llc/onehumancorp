use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Schema, Set};

use super::{connection::AppDatabase, entities};

pub const CORE_SCHEMA_VERSION: &str = "20260730_000001_portable_core";

pub async fn migrate(database: &AppDatabase) -> Result<(), sea_orm::DbErr> {
    let connection = database.connection();
    let backend = connection.get_database_backend();
    let schema = Schema::new(backend);

    let mut versions = schema.create_table_from_entity(entities::schema_version::Entity);
    versions.if_not_exists();
    connection.execute(backend.build(&versions)).await?;

    let mut users = schema.create_table_from_entity(entities::user::Entity);
    users.if_not_exists();
    connection.execute(backend.build(&users)).await?;

    let mut products = schema.create_table_from_entity(entities::product::Entity);
    products.if_not_exists();
    connection.execute(backend.build(&products)).await?;

    if entities::schema_version::Entity::find_by_id(CORE_SCHEMA_VERSION)
        .one(connection)
        .await?
        .is_none()
    {
        entities::schema_version::ActiveModel {
            id: Set(CORE_SCHEMA_VERSION.to_owned()),
            applied_at: Set(Utc::now()),
        }
        .insert(connection)
        .await?;
    }
    Ok(())
}
