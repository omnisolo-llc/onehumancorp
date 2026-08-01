use chrono::Utc;
use sea_orm::sea_query::Index;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, EntityTrait, QueryOrder, Schema, Set, Statement,
    TransactionTrait,
};

use super::{capabilities::DatabaseBackend as AppBackend, connection::AppDatabase, entities};
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

    let mut identity_email_claims =
        schema.create_table_from_entity(auth_entities::identity_email_claim::Entity);
    identity_email_claims.if_not_exists();
    connection
        .execute(backend.build(&identity_email_claims))
        .await?;

    let mut revoked_tokens = schema.create_table_from_entity(auth_entities::revoked_token::Entity);
    revoked_tokens.if_not_exists();
    connection.execute(backend.build(&revoked_tokens)).await?;

    let indexes = [
        (
            "users",
            "ux_users_tenant_username",
            Index::create()
                .name("ux_users_tenant_username")
                .table(auth_entities::user::Entity)
                .col(auth_entities::user::Column::TenantId)
                .col(auth_entities::user::Column::Username)
                .unique()
                .if_not_exists()
                .to_owned(),
        ),
        (
            "users",
            "ux_users_tenant_email",
            Index::create()
                .name("ux_users_tenant_email")
                .table(auth_entities::user::Entity)
                .col(auth_entities::user::Column::TenantId)
                .col(auth_entities::user::Column::Email)
                .unique()
                .if_not_exists()
                .to_owned(),
        ),
        (
            "registration_tickets",
            "ux_registration_tickets_token_hash",
            Index::create()
                .name("ux_registration_tickets_token_hash")
                .table(auth_entities::registration_ticket::Entity)
                .col(auth_entities::registration_ticket::Column::TokenHash)
                .unique()
                .if_not_exists()
                .to_owned(),
        ),
        (
            "registration_invitations",
            "ux_registration_invitations_token_hash",
            Index::create()
                .name("ux_registration_invitations_token_hash")
                .table(auth_entities::invitation::Entity)
                .col(auth_entities::invitation::Column::TokenHash)
                .unique()
                .if_not_exists()
                .to_owned(),
        ),
        (
            "external_identities",
            "ux_external_identities_provider_subject",
            Index::create()
                .name("ux_external_identities_provider_subject")
                .table(auth_entities::external_identity::Entity)
                .col(auth_entities::external_identity::Column::ProviderKey)
                .col(auth_entities::external_identity::Column::Issuer)
                .col(auth_entities::external_identity::Column::Subject)
                .unique()
                .if_not_exists()
                .to_owned(),
        ),
    ];
    for (table_name, index_name, index) in indexes {
        if database.backend() == AppBackend::MySql
            && mysql_index_exists(connection, table_name, index_name).await?
        {
            continue;
        }
        connection.execute(backend.build(&index)).await?;
    }

    backfill_identity_email_claims(connection).await?;

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

async fn backfill_identity_email_claims(
    connection: &sea_orm::DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    let transaction = connection.begin().await?;
    let users = auth_entities::user::Entity::find()
        .order_by_asc(auth_entities::user::Column::Id)
        .all(&transaction)
        .await?;
    let mut expected = Vec::with_capacity(users.len());
    for user in users {
        let normalized_email = server_auth::validation::normalize_email(&user.email).map_err(|_| {
            sea_orm::DbErr::Custom(format!(
                "existing identity has an invalid email: user {}",
                user.id
            ))
        })?;
        expected.push((normalized_email, user.id, user.created_at));
    }
    expected.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    for pair in expected.windows(2) {
        if pair[0].0 == pair[1].0 && pair[0].1 != pair[1].1 {
            return Err(identity_email_collision(&pair[0].0));
        }
    }

    let existing = auth_entities::identity_email_claim::Entity::find()
        .all(&transaction)
        .await?;
    let existing = existing
        .into_iter()
        .map(|claim| (claim.normalized_email, claim.user_id))
        .collect::<std::collections::HashMap<_, _>>();
    for (normalized_email, user_id, claimed_at) in expected {
        if let Some(owner) = existing.get(&normalized_email) {
            if owner != &user_id {
                return Err(identity_email_collision(&normalized_email));
            }
            continue;
        }
        auth_entities::identity_email_claim::ActiveModel {
            normalized_email: Set(normalized_email),
            user_id: Set(user_id),
            claimed_at: Set(claimed_at),
        }
        .insert(&transaction)
        .await?;
    }
    transaction.commit().await
}

fn identity_email_collision(normalized_email: &str) -> sea_orm::DbErr {
    sea_orm::DbErr::Custom(format!(
        "identity email collision for normalized email {normalized_email}"
    ))
}

async fn mysql_index_exists(
    connection: &sea_orm::DatabaseConnection,
    table_name: &str,
    index_name: &str,
) -> Result<bool, sea_orm::DbErr> {
    connection
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::MySql,
            "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = ? AND index_name = ? LIMIT 1",
            [table_name.into(), index_name.into()],
        ))
        .await
        .map(|row| row.is_some())
}
