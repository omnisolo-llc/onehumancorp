use chrono::Utc;
use sea_orm::sea_query::Index;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Schema, Set};

use super::{connection::AppDatabase, entities};
use server_auth::seaorm_store::entities as auth_entities;

pub const CORE_SCHEMA_VERSION: &str = "20260730_000001_portable_core";
pub const AUTH_SCHEMA_VERSION: &str = "20260730_000002_auth_registration";

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

    let mut settings = schema.create_table_from_entity(auth_entities::application_setting::Entity);
    settings.if_not_exists();
    connection.execute(backend.build(&settings)).await?;

    let mut challenges = schema.create_table_from_entity(auth_entities::email_challenge::Entity);
    challenges.if_not_exists();
    connection.execute(backend.build(&challenges)).await?;

    let mut tickets = schema.create_table_from_entity(auth_entities::registration_ticket::Entity);
    tickets.if_not_exists();
    connection.execute(backend.build(&tickets)).await?;

    let mut invitations = schema.create_table_from_entity(auth_entities::invitation::Entity);
    invitations.if_not_exists();
    connection.execute(backend.build(&invitations)).await?;

    let mut oidc_providers = schema.create_table_from_entity(auth_entities::oidc_provider::Entity);
    oidc_providers.if_not_exists();
    connection.execute(backend.build(&oidc_providers)).await?;

    let mut external_identities =
        schema.create_table_from_entity(auth_entities::external_identity::Entity);
    external_identities.if_not_exists();
    connection
        .execute(backend.build(&external_identities))
        .await?;

    let mut revoked_tokens = schema.create_table_from_entity(auth_entities::revoked_token::Entity);
    revoked_tokens.if_not_exists();
    connection.execute(backend.build(&revoked_tokens)).await?;

    let indexes = [
        Index::create()
            .name("ux_users_tenant_username")
            .table(auth_entities::user::Entity)
            .col(auth_entities::user::Column::TenantId)
            .col(auth_entities::user::Column::Username)
            .unique()
            .if_not_exists()
            .to_owned(),
        Index::create()
            .name("ux_users_tenant_email")
            .table(auth_entities::user::Entity)
            .col(auth_entities::user::Column::TenantId)
            .col(auth_entities::user::Column::Email)
            .unique()
            .if_not_exists()
            .to_owned(),
        Index::create()
            .name("ux_registration_tickets_token_hash")
            .table(auth_entities::registration_ticket::Entity)
            .col(auth_entities::registration_ticket::Column::TokenHash)
            .unique()
            .if_not_exists()
            .to_owned(),
        Index::create()
            .name("ux_registration_invitations_token_hash")
            .table(auth_entities::invitation::Entity)
            .col(auth_entities::invitation::Column::TokenHash)
            .unique()
            .if_not_exists()
            .to_owned(),
        Index::create()
            .name("ux_external_identities_provider_subject")
            .table(auth_entities::external_identity::Entity)
            .col(auth_entities::external_identity::Column::ProviderKey)
            .col(auth_entities::external_identity::Column::Issuer)
            .col(auth_entities::external_identity::Column::Subject)
            .unique()
            .if_not_exists()
            .to_owned(),
    ];
    for index in indexes {
        connection.execute(backend.build(&index)).await?;
    }

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

    if auth_entities::application_setting::Entity::find_by_id("registration_mode")
        .one(connection)
        .await?
        .is_none()
    {
        auth_entities::application_setting::ActiveModel {
            key: Set("registration_mode".to_string()),
            value: Set("closed".to_string()),
            updated_at: Set(Utc::now()),
            updated_by: Set(None),
        }
        .insert(connection)
        .await?;
    }

    if entities::schema_version::Entity::find_by_id(AUTH_SCHEMA_VERSION)
        .one(connection)
        .await?
        .is_none()
    {
        entities::schema_version::ActiveModel {
            id: Set(AUTH_SCHEMA_VERSION.to_owned()),
            applied_at: Set(Utc::now()),
        }
        .insert(connection)
        .await?;
    }
    Ok(())
}
