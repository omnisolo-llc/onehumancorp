use chrono::Utc;
use sea_orm::sea_query::Index;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryOrder, Schema, Set,
    Statement, TransactionTrait,
};

use super::{capabilities::DatabaseBackend as AppBackend, connection::AppDatabase, entities};
use server_auth::seaorm_store::entities as auth_entities;

pub const CORE_SCHEMA_VERSION: &str = "20260730_000001_portable_core";
pub const AUTH_SCHEMA_VERSION: &str = "20260730_000002_auth_registration";
pub const PORTABLE_ROLE_SCHEMA_VERSION: &str = "20260801_000003_portable_user_roles";
const POSTGRES_MIGRATION_LOCK_KEY: i64 = 0x4f48_435f_4d49_4752;

pub async fn migrate(database: &AppDatabase) -> Result<(), sea_orm::DbErr> {
    let connection = database.connection();
    if connection.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
        let migration_guard = acquire_postgres_migration_guard(connection).await?;
        migrate_with_connection(database.backend(), &migration_guard).await?;
        migration_guard.commit().await?;
        return Ok(());
    }
    migrate_with_connection(database.backend(), connection).await
}

async fn migrate_with_connection<C>(
    app_backend: AppBackend,
    connection: &C,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let backend = connection.get_database_backend();
    let schema = Schema::new(backend);

    let mut versions = schema.create_table_from_entity(entities::schema_version::Entity);
    versions.if_not_exists();
    connection.execute(backend.build(&versions)).await?;

    let mut users = schema.create_table_from_entity(entities::user::Entity);
    users.if_not_exists();
    connection.execute(backend.build(&users)).await?;
    ensure_legacy_role_column(connection).await?;

    let mut user_roles = schema.create_table_from_entity(auth_entities::identity_user_role::Entity);
    user_roles.if_not_exists();
    connection.execute(backend.build(&user_roles)).await?;

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
            "email_verification_challenges",
            "ux_email_verification_challenges_email",
            Index::create()
                .name("ux_email_verification_challenges_email")
                .table(auth_entities::email_challenge::Entity)
                .col(auth_entities::email_challenge::Column::Email)
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
        if app_backend == AppBackend::MySql
            && mysql_index_exists(connection, table_name, index_name).await?
        {
            continue;
        }
        connection.execute(backend.build(&index)).await?;
    }

    backfill_portable_user_roles(connection).await?;
    backfill_identity_email_claims(connection).await?;
    configure_postgres_role_rls(connection).await?;

    insert_default_or_ignore(
        connection,
        "onehumancorp_schema_versions",
        &["id", "applied_at"],
        vec![CORE_SCHEMA_VERSION.to_owned().into(), Utc::now().into()],
    )
    .await?;

    insert_default_or_ignore(
        connection,
        "application_settings",
        &["key", "value", "updated_at", "updated_by"],
        vec![
            "registration_mode".to_owned().into(),
            "closed".to_owned().into(),
            Utc::now().into(),
            sea_orm::Value::String(None),
        ],
    )
    .await?;

    insert_default_or_ignore(
        connection,
        "onehumancorp_schema_versions",
        &["id", "applied_at"],
        vec![AUTH_SCHEMA_VERSION.to_owned().into(), Utc::now().into()],
    )
    .await?;

    insert_default_or_ignore(
        connection,
        "onehumancorp_schema_versions",
        &["id", "applied_at"],
        vec![
            PORTABLE_ROLE_SCHEMA_VERSION.to_owned().into(),
            Utc::now().into(),
        ],
    )
    .await?;
    Ok(())
}

async fn insert_default_or_ignore<C>(
    connection: &C,
    table: &str,
    columns: &[&str],
    values: Vec<sea_orm::Value>,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let backend = connection.get_database_backend();
    let quote = |identifier: &str| match backend {
        sea_orm::DatabaseBackend::MySql => format!("`{identifier}`"),
        _ => format!("\"{identifier}\""),
    };
    let placeholders: Vec<String> = match backend {
        sea_orm::DatabaseBackend::Postgres => (1..=values.len())
            .map(|index| format!("${index}"))
            .collect(),
        _ => vec!["?".to_string(); values.len()],
    };
    let keyword = match backend {
        sea_orm::DatabaseBackend::MySql => "INSERT IGNORE",
        sea_orm::DatabaseBackend::Sqlite => "INSERT OR IGNORE",
        _ => "INSERT",
    };
    let mut sql = format!(
        "{keyword} INTO {} ({}) VALUES ({})",
        quote(table),
        columns
            .iter()
            .map(|column| quote(column))
            .collect::<Vec<_>>()
            .join(", "),
        placeholders.join(", ")
    );
    if backend == sea_orm::DatabaseBackend::Postgres {
        sql.push_str(" ON CONFLICT DO NOTHING");
    }
    connection
        .execute(Statement::from_sql_and_values(backend, sql, values))
        .await?;
    Ok(())
}

async fn acquire_postgres_migration_guard(
    connection: &sea_orm::DatabaseConnection,
) -> Result<DatabaseTransaction, sea_orm::DbErr> {
    let transaction = connection.begin().await?;
    transaction
        .execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT pg_advisory_xact_lock($1)",
            [POSTGRES_MIGRATION_LOCK_KEY.into()],
        ))
        .await?;
    Ok(transaction)
}

async fn ensure_legacy_role_column<C: ConnectionTrait>(
    connection: &C,
) -> Result<(), sea_orm::DbErr> {
    let backend = connection.get_database_backend();
    match backend {
        sea_orm::DatabaseBackend::Postgres => {
            connection
                .execute(Statement::from_string(
                    backend,
                    "ALTER TABLE users ADD COLUMN IF NOT EXISTS roles TEXT[] DEFAULT '{}'"
                        .to_string(),
                ))
                .await?;
        }
        sea_orm::DatabaseBackend::MySql => {
            if !mysql_column_exists(connection, "users", "roles").await? {
                connection
                    .execute(Statement::from_string(
                        backend,
                        "ALTER TABLE users ADD COLUMN roles JSON".to_string(),
                    ))
                    .await?;
            }
        }
        sea_orm::DatabaseBackend::Sqlite => {
            let columns = connection
                .query_all(Statement::from_string(
                    backend,
                    "PRAGMA table_info(users)".to_string(),
                ))
                .await?;
            let has_roles = columns.iter().any(|column| {
                column
                    .try_get::<String>("", "name")
                    .is_ok_and(|name| name == "roles")
            });
            if !has_roles {
                connection
                    .execute(Statement::from_string(
                        backend,
                        "ALTER TABLE users ADD COLUMN roles TEXT NOT NULL DEFAULT '[]'".to_string(),
                    ))
                    .await?;
            }
        }
    }
    Ok(())
}

async fn configure_postgres_role_rls<C: ConnectionTrait>(
    connection: &C,
) -> Result<(), sea_orm::DbErr> {
    if connection.get_database_backend() != sea_orm::DatabaseBackend::Postgres {
        return Ok(());
    }
    for sql in [
        "ALTER TABLE identity_user_roles ENABLE ROW LEVEL SECURITY",
        "ALTER TABLE identity_user_roles FORCE ROW LEVEL SECURITY",
        "DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'identity_user_roles' AND policyname = 'tenant_isolation_identity_user_roles') THEN CREATE POLICY tenant_isolation_identity_user_roles ON identity_user_roles USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)); END IF; END $$",
    ] {
        connection
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                sql.to_string(),
            ))
            .await?;
    }
    Ok(())
}

async fn backfill_portable_user_roles<C>(connection: &C) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let backend = connection.get_database_backend();
    let (transaction, assumed_bypass_role) =
        begin_portable_migration_transaction(connection).await?;
    let sql = match backend {
        sea_orm::DatabaseBackend::Postgres => {
            "INSERT INTO identity_user_roles (user_id, role_name, tenant_id, position) SELECT users.id, role_name, users.tenant_id, position::INTEGER - 1 FROM users CROSS JOIN LATERAL unnest(COALESCE(users.roles, ARRAY[]::TEXT[])) WITH ORDINALITY AS legacy_roles(role_name, position) ON CONFLICT (user_id, role_name) DO NOTHING"
        }
        sea_orm::DatabaseBackend::MySql => {
            "INSERT IGNORE INTO identity_user_roles (user_id, role_name, tenant_id, position) SELECT users.id, legacy_roles.role_name, users.tenant_id, legacy_roles.position - 1 FROM users CROSS JOIN JSON_TABLE(COALESCE(users.roles, JSON_ARRAY()), '$[*]' COLUMNS(position FOR ORDINALITY, role_name VARCHAR(255) PATH '$')) AS legacy_roles"
        }
        sea_orm::DatabaseBackend::Sqlite => {
            "INSERT OR IGNORE INTO identity_user_roles (user_id, role_name, tenant_id, position) SELECT users.id, CAST(legacy_roles.value AS TEXT), users.tenant_id, CAST(legacy_roles.key AS INTEGER) FROM users CROSS JOIN json_each(COALESCE(users.roles, '[]')) AS legacy_roles"
        }
    };
    transaction
        .execute(Statement::from_string(backend, sql.to_string()))
        .await?;
    reset_portable_migration_role(&transaction, assumed_bypass_role).await?;
    transaction.commit().await
}

async fn backfill_identity_email_claims<C>(connection: &C) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let (transaction, assumed_bypass_role) =
        begin_portable_migration_transaction(connection).await?;
    let users = auth_entities::user::Entity::find()
        .order_by_asc(auth_entities::user::Column::Id)
        .all(&transaction)
        .await?;
    let mut expected = Vec::with_capacity(users.len());
    for user in users {
        let normalized_email =
            server_auth::validation::normalize_email(&user.email).map_err(|_| {
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
    reset_portable_migration_role(&transaction, assumed_bypass_role).await?;
    transaction.commit().await
}

async fn begin_portable_migration_transaction<C>(
    connection: &C,
) -> Result<(DatabaseTransaction, bool), sea_orm::DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let transaction = connection.begin().await?;
    if connection.get_database_backend() != sea_orm::DatabaseBackend::Postgres {
        return Ok((transaction, false));
    }

    let bypass_role_is_usable = transaction
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ohc_bypassrls' AND rolbypassrls AND pg_has_role(current_user, rolname, 'MEMBER')) AS bypass_role_is_usable"
                .to_string(),
        ))
        .await?
        .ok_or_else(|| sea_orm::DbErr::Custom("PostgreSQL role check returned no row".to_string()))?
        .try_get::<bool>("", "bypass_role_is_usable")?;
    if bypass_role_is_usable {
        transaction
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                "SET LOCAL ROLE ohc_bypassrls".to_string(),
            ))
            .await?;
        return Ok((transaction, true));
    }

    let users_rls_is_active = transaction
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT relrowsecurity, row_security_active(oid) AS row_security_active FROM pg_class WHERE oid = to_regclass('users')"
                .to_string(),
        ))
        .await?
        .is_some_and(|row| row.try_get::<bool>("", "row_security_active").unwrap_or(true));
    if users_rls_is_active {
        return Err(sea_orm::DbErr::Custom(
            "PostgreSQL users RLS is active and requires membership in the ohc_bypassrls migration role"
                .to_string(),
        ));
    }
    Ok((transaction, false))
}

async fn reset_portable_migration_role(
    transaction: &DatabaseTransaction,
    assumed_bypass_role: bool,
) -> Result<(), sea_orm::DbErr> {
    if assumed_bypass_role {
        transaction
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                "SET LOCAL ROLE NONE".to_string(),
            ))
            .await?;
    }
    Ok(())
}

fn identity_email_collision(normalized_email: &str) -> sea_orm::DbErr {
    sea_orm::DbErr::Custom(format!(
        "identity email collision for normalized email {normalized_email}"
    ))
}

async fn mysql_index_exists<C: ConnectionTrait>(
    connection: &C,
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

async fn mysql_column_exists<C: ConnectionTrait>(
    connection: &C,
    table_name: &str,
    column_name: &str,
) -> Result<bool, sea_orm::DbErr> {
    connection
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::MySql,
            "SELECT 1 FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = ? AND column_name = ? LIMIT 1",
            [table_name.into(), column_name.into()],
        ))
        .await
        .map(|row| row.is_some())
}
