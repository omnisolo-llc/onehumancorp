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
    // Deployment migrations install accounting tables without granting a budget.
    // Runtime usage handlers never require schema ownership or DDL privileges.
    match backend {
        sea_orm::DatabaseBackend::Postgres => {
            connection
                .execute_unprepared(include_str!("usage_ledger_postgres.sql"))
                .await?;
        }
        sea_orm::DatabaseBackend::Sqlite => {
            connection
                .execute_unprepared(include_str!("usage_ledger_sqlite.sql"))
                .await?;
        }
        _ => {}
    }
    let schema = Schema::new(backend);

    let mut versions = schema.create_table_from_entity(entities::schema_version::Entity);
    versions.if_not_exists();
    connection.execute(backend.build(&versions)).await?;

    // Registration must not create a reduced table that prevents the canonical
    // bootstrap from installing currency, plan and subscription defaults later.
    // Existing tables and their paid data remain untouched.
    let tenant_schema = match backend {
        sea_orm::DatabaseBackend::Sqlite => include_str!("tenant_schema_sqlite.sql"),
        sea_orm::DatabaseBackend::MySql => include_str!("tenant_schema_mysql.sql"),
        sea_orm::DatabaseBackend::Postgres => include_str!("tenant_schema_postgres.sql"),
    };
    connection.execute_unprepared(tenant_schema).await?;

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
    ensure_registration_source_columns(connection).await?;

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
    if backend == sea_orm::DatabaseBackend::Sqlite {
        connection
            .execute_unprepared(include_str!("tenant_execution_receipts_sqlite.sql"))
            .await?;
    }
    if backend == sea_orm::DatabaseBackend::Postgres {
        connection
            .execute_unprepared(include_str!("token_revocation_fence_postgres.sql"))
            .await?;
    }
    if backend == sea_orm::DatabaseBackend::Sqlite {
        connection
            .execute_unprepared(include_str!("sms_verification_sqlite.sql"))
            .await?;
    }
    configure_agent_definition_authority(connection).await?;

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

/// Private identity-derived public eligibility. This runs under the existing
/// portable migration lock/authority, before any application routes are built.
async fn configure_agent_definition_authority<C>(connection: &C) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let backend = connection.get_database_backend();
    if backend == sea_orm::DatabaseBackend::MySql {
        return Ok(());
    }
    let (transaction, assumed_bypass_role) =
        begin_portable_migration_transaction(connection).await?;
    if backend == sea_orm::DatabaseBackend::Postgres {
        let sql = include_str!("agent_definition_authority_pg.sql");
        let (schema, rest) = sql
            .split_once("-- AUTHORITY_SCHEMA_END")
            .ok_or_else(|| sea_orm::DbErr::Custom("authority schema boundary missing".into()))?;
        let (backfill, triggers) = rest
            .split_once("-- AUTHORITY_BACKFILL_END")
            .ok_or_else(|| sea_orm::DbErr::Custom("authority backfill boundary missing".into()))?;
        // DDL stays with the existing migration owner; only the private-row
        // backfill uses the already-validated migration authority, as other
        // portable identity backfills do. Runtime never assumes that role.
        reset_portable_migration_role(&transaction, assumed_bypass_role).await?;
        transaction.execute_unprepared(schema).await?;
        if assumed_bypass_role {
            transaction
                .execute_unprepared("SET LOCAL ROLE ohc_bypassrls")
                .await?;
        }
        transaction.execute_unprepared(backfill).await?;
        reset_portable_migration_role(&transaction, assumed_bypass_role).await?;
        transaction.execute_unprepared(triggers).await?;
    } else {
        let enabled = transaction
            .query_one(Statement::from_string(
                backend,
                "PRAGMA foreign_keys".to_string(),
            ))
            .await?
            .ok_or_else(|| sea_orm::DbErr::Custom("SQLite foreign key state unavailable".into()))?
            .try_get::<i64>("", "foreign_keys")?;
        if enabled != 1 {
            return Err(sea_orm::DbErr::Custom(
                "SQLite foreign keys must be enabled".into(),
            ));
        }
        let columns = transaction
            .query_all(Statement::from_string(
                backend,
                "PRAGMA table_info(users)".to_string(),
            ))
            .await?;
        for (name, ddl) in [
            (
                "marketplace_authority_key",
                "ALTER TABLE users ADD COLUMN marketplace_authority_key TEXT",
            ),
            (
                "marketplace_eligible",
                "ALTER TABLE users ADD COLUMN marketplace_eligible BOOLEAN NOT NULL DEFAULT 0",
            ),
        ] {
            if !columns.iter().any(|row| {
                row.try_get::<String>("", "name")
                    .is_ok_and(|value| value == name)
            }) {
                transaction.execute_unprepared(ddl).await?;
            }
        }
        transaction
            .execute_unprepared(include_str!("agent_definition_authority_sqlite.sql"))
            .await?;
    }
    transaction.commit().await
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

/// Existing consumed tickets deliberately remain unbound: mutable email addresses
/// are not evidence of the user/tenant that originally completed registration.
async fn ensure_registration_source_columns<C: ConnectionTrait>(
    connection: &C,
) -> Result<(), sea_orm::DbErr> {
    let backend = connection.get_database_backend();
    for column in ["consumed_by_user_id", "consumed_by_tenant_id"] {
        let exists = match backend {
            sea_orm::DatabaseBackend::Postgres => false,
            sea_orm::DatabaseBackend::MySql => {
                mysql_column_exists(connection, "registration_tickets", column).await?
            }
            sea_orm::DatabaseBackend::Sqlite => connection
                .query_all(Statement::from_string(
                    backend,
                    "PRAGMA table_info(registration_tickets)".to_string(),
                ))
                .await?
                .iter()
                .any(|row| {
                    row.try_get::<String>("", "name")
                        .is_ok_and(|name| name == column)
                }),
        };
        if !exists {
            let guard = if backend == sea_orm::DatabaseBackend::Postgres {
                "IF NOT EXISTS "
            } else {
                ""
            };
            connection
                .execute(Statement::from_string(
                    backend,
                    format!(
                        "ALTER TABLE registration_tickets ADD COLUMN {guard}{column} TEXT NULL"
                    ),
                ))
                .await?;
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
    // The normalized table becomes authoritative after its first conversion.
    // Claim the existing schema-version row in this same transaction so a
    // concurrent/repeated startup cannot restore roles from a stale mirror.
    let marker_sql = match backend {
        sea_orm::DatabaseBackend::Postgres => {
            "INSERT INTO onehumancorp_schema_versions(id,applied_at) VALUES($1,$2) ON CONFLICT(id) DO NOTHING"
        }
        sea_orm::DatabaseBackend::MySql => {
            "INSERT IGNORE INTO onehumancorp_schema_versions(id,applied_at) VALUES(?,?)"
        }
        sea_orm::DatabaseBackend::Sqlite => {
            "INSERT OR IGNORE INTO onehumancorp_schema_versions(id,applied_at) VALUES(?,?)"
        }
    };
    let claimed = transaction
        .execute(Statement::from_sql_and_values(
            backend,
            marker_sql,
            [
                PORTABLE_ROLE_SCHEMA_VERSION.to_owned().into(),
                Utc::now().into(),
            ],
        ))
        .await?;
    if claimed.rows_affected() == 0 {
        reset_portable_migration_role(&transaction, assumed_bypass_role).await?;
        return transaction.commit().await;
    }
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

#[cfg(test)]
mod registration_source_tests {
    use super::AppDatabase;
    use chrono::Utc;
    use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, Statement};
    use server_auth::seaorm_store::SeaOrmAuthRepository;
    use server_auth::seaorm_store::entities as auth_entities;

    async fn execute(db: &AppDatabase, sql: &str) {
        db.connection().execute_unprepared(sql).await.unwrap();
    }
    fn user() -> server_auth::User {
        let now = Utc::now();
        server_auth::User {
            id: "new-source-owner".into(),
            username: "new-source-owner".into(),
            email: "new-source-owner@example.test".into(),
            password_hash: "test-owned-unused".into(),
            active: true,
            roles: vec![server_auth::ROLE_ADMIN.into()],
            organization_id: Some("new-source-tenant".into()),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        }
    }
    async fn fresh() -> AppDatabase {
        let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
        super::migrate(&db).await.unwrap();
        execute(
            &db,
            "UPDATE application_settings SET value='open' WHERE key='registration_mode'",
        )
        .await;
        execute(&db, "INSERT INTO registration_tickets(id,email,token_hash,issued_at,expires_at,consumed_at,invitation_id) VALUES('new-ticket','new-source-owner@example.test','ticket-hash',CURRENT_TIMESTAMP,datetime('now','+1 day'),NULL,NULL)").await;
        db
    }
    async fn receipt(db: &AppDatabase, id: &str) -> (Option<String>, Option<String>) {
        let row = db.connection().query_one(Statement::from_sql_and_values(DatabaseBackend::Sqlite,
            "SELECT consumed_by_user_id, consumed_by_tenant_id FROM registration_tickets WHERE id=?", [id.into()]))
            .await.expect("actual migration must install immutable source identity columns").unwrap();
        (
            row.try_get("", "consumed_by_user_id").unwrap(),
            row.try_get("", "consumed_by_tenant_id").unwrap(),
        )
    }
    fn canonical_tenant_sql(backend: DatabaseBackend) -> &'static str {
        let source = match backend {
            DatabaseBackend::Postgres => include_str!("../migrations/001_initial.sql"),
            _ => include_str!("../db.rs"),
        };
        let index = usize::from(backend == DatabaseBackend::MySql);
        let start = source
            .match_indices("CREATE TABLE IF NOT EXISTS tenants (")
            .nth(index)
            .expect("canonical tenant schema must exist")
            .0;
        let end = start + source[start..].find(");").unwrap() + 2;
        &source[start..end]
    }

    async fn tenant_columns(db: &AppDatabase) -> Vec<(String, String, i64, Option<String>, i64)> {
        let rows = db
            .connection()
            .query_all(Statement::from_string(
                DatabaseBackend::Sqlite,
                "PRAGMA table_info(tenants)".to_owned(),
            ))
            .await
            .unwrap();
        let mut columns: Vec<_> = rows
            .into_iter()
            .map(|row| {
                (
                    row.try_get("", "name").unwrap(),
                    row.try_get("", "type").unwrap(),
                    row.try_get("", "notnull").unwrap(),
                    row.try_get("", "dflt_value").unwrap(),
                    row.try_get("", "pk").unwrap(),
                )
            })
            .collect();
        columns.sort();
        columns
    }

    #[tokio::test]
    async fn portable_first_initialization_preserves_the_complete_canonical_tenant_schema() {
        let reference = AppDatabase::connect("sqlite::memory:").await.unwrap();
        execute(&reference, canonical_tenant_sql(DatabaseBackend::Sqlite)).await;
        let expected = tenant_columns(&reference).await;
        let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
        super::migrate(&db).await.unwrap();
        // This is the unchanged DB::run_migrations statement, applied afterwards.
        // CREATE IF NOT EXISTS must not leave a reduced portable-first table.
        execute(&db, canonical_tenant_sql(DatabaseBackend::Sqlite)).await;
        assert_eq!(tenant_columns(&db).await, expected);
        execute(
            &db,
            "INSERT INTO tenants(id,name) VALUES('schema-defaults','Schema defaults')",
        )
        .await;
        let row=db.connection().query_one(Statement::from_string(DatabaseBackend::Sqlite,
            "SELECT owner_id, tier, plan_tier, has_claimed_trial_extension, default_currency, is_subscribable, subscription_frequency, subscription_discount_percent, _sync_status, version FROM tenants WHERE id='schema-defaults'".to_owned())).await.unwrap().unwrap();
        assert_eq!(row.try_get::<Option<String>>("", "owner_id").unwrap(), None);
        assert_eq!(row.try_get::<String>("", "tier").unwrap(), "free");
        assert_eq!(row.try_get::<String>("", "plan_tier").unwrap(), "free");
        assert!(
            !row.try_get::<bool>("", "has_claimed_trial_extension")
                .unwrap()
        );
        assert_eq!(
            row.try_get::<String>("", "default_currency").unwrap(),
            "USD"
        );
        assert!(!row.try_get::<bool>("", "is_subscribable").unwrap());
        assert_eq!(
            row.try_get::<Option<String>>("", "subscription_frequency")
                .unwrap(),
            None
        );
        assert_eq!(
            row.try_get::<i64>("", "subscription_discount_percent")
                .unwrap(),
            0
        );
        assert_eq!(
            row.try_get::<String>("", "_sync_status").unwrap(),
            "pending"
        );
        assert_eq!(row.try_get::<i64>("", "version").unwrap(), 1);
    }

    #[tokio::test]
    async fn canonical_first_initialization_preserves_existing_paid_tenant_settings() {
        let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
        execute(&db, canonical_tenant_sql(DatabaseBackend::Sqlite)).await;
        execute(&db, "INSERT INTO tenants(id,owner_id,name,tier,plan_tier,default_currency,has_claimed_trial_extension,subscription_frequency) VALUES('existing','existing-owner','Existing business','pro','business','EUR',true,'month')").await;
        let columns = tenant_columns(&db).await;
        super::migrate(&db).await.unwrap();
        super::migrate(&db).await.unwrap();
        assert_eq!(tenant_columns(&db).await, columns);
        let row=db.connection().query_one(Statement::from_string(DatabaseBackend::Sqlite,
            "SELECT owner_id,name,tier,plan_tier,default_currency,has_claimed_trial_extension,subscription_frequency FROM tenants WHERE id='existing'".to_owned())).await.unwrap().unwrap();
        for (key, value) in [
            ("owner_id", "existing-owner"),
            ("name", "Existing business"),
            ("tier", "pro"),
            ("plan_tier", "business"),
            ("default_currency", "EUR"),
            ("subscription_frequency", "month"),
        ] {
            assert_eq!(row.try_get::<String>("", key).unwrap(), value);
        }
        assert!(
            row.try_get::<bool>("", "has_claimed_trial_extension")
                .unwrap()
        );
    }

    #[test]
    fn portable_tenant_definitions_match_each_complete_canonical_bootstrap() {
        for (backend, portable) in [
            (
                DatabaseBackend::Sqlite,
                include_str!("tenant_schema_sqlite.sql"),
            ),
            (
                DatabaseBackend::MySql,
                include_str!("tenant_schema_mysql.sql"),
            ),
            (
                DatabaseBackend::Postgres,
                include_str!("tenant_schema_postgres.sql"),
            ),
        ] {
            assert_eq!(
                portable.split_whitespace().collect::<Vec<_>>(),
                canonical_tenant_sql(backend)
                    .split_whitespace()
                    .collect::<Vec<_>>(),
                "portable tenant columns/defaults must track the actual backend bootstrap"
            );
        }
    }

    #[tokio::test]
    async fn upgrading_old_tickets_preserves_history_without_inventing_source_owners() {
        let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
        execute(&db, "CREATE TABLE registration_tickets(id TEXT PRIMARY KEY,email TEXT NOT NULL,token_hash TEXT NOT NULL,issued_at TEXT NOT NULL,expires_at TEXT NOT NULL,consumed_at TEXT,invitation_id TEXT)").await;
        execute(&db, "INSERT INTO registration_tickets VALUES('old-consumed','old@example.test','old-hash','2026-01-01','2026-01-02','2026-01-01',NULL)").await;
        super::migrate(&db).await.unwrap();
        assert_eq!(receipt(&db, "old-consumed").await, (None, None));
        super::migrate(&db).await.unwrap();
        assert_eq!(receipt(&db, "old-consumed").await, (None, None));
        let row = db
            .connection()
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT email, consumed_at FROM registration_tickets WHERE id='old-consumed'"
                    .to_string(),
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            row.try_get::<String>("", "email").unwrap(),
            "old@example.test"
        );
        assert_eq!(
            row.try_get::<String>("", "consumed_at").unwrap(),
            "2026-01-01"
        );
    }
    #[tokio::test]
    async fn migrated_registration_binds_the_source_to_the_actual_created_user_and_tenant() {
        let db = fresh().await;
        let repository = SeaOrmAuthRepository::new(db.connection().clone());
        let created = repository
            .consume_ticket_and_create_user("ticket-hash", Utc::now(), user())
            .await
            .unwrap();
        let tenant =
            auth_entities::tenant::Entity::find_by_id(created.organization_id.clone().unwrap())
                .one(db.connection())
                .await
                .unwrap()
                .unwrap();
        assert_eq!(tenant.name, created.username);
        assert_eq!(tenant.tier, "free");
        assert_eq!(
            receipt(&db, "new-ticket").await,
            (Some(created.id), created.organization_id)
        );
    }
    #[tokio::test]
    async fn source_receipt_failure_rolls_back_user_roles_email_claim_and_ticket_consumption() {
        let db = fresh().await;
        execute(&db, "CREATE TRIGGER deny_source_receipt BEFORE UPDATE OF consumed_by_user_id ON registration_tickets BEGIN SELECT RAISE(ABORT,'owned fixture denies source receipt'); END").await;
        let repository = SeaOrmAuthRepository::new(db.connection().clone());
        assert!(
            repository
                .consume_ticket_and_create_user("ticket-hash", Utc::now(), user())
                .await
                .is_err(),
            "source identity must be part of the actual registration transaction"
        );
        for table in [
            "tenants",
            "users",
            "identity_user_roles",
            "identity_email_claims",
        ] {
            let row = db
                .connection()
                .query_one(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    format!("SELECT COUNT(*) AS count FROM {table}"),
                ))
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                row.try_get::<i64>("", "count").unwrap(),
                0,
                "{table} must roll back"
            );
        }
        let row = db
            .connection()
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT consumed_at FROM registration_tickets WHERE id='new-ticket'".to_string(),
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            row.try_get::<Option<String>>("", "consumed_at").unwrap(),
            None
        );
    }
}
