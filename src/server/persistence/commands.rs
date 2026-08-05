use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use super::{
    connection::{AppDatabase, DatabaseUrl},
    entities, migration,
};

type CommandResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub fn database_url_from_environment() -> CommandResult<DatabaseUrl> {
    let direct = std::env::var("DATABASE_URL").ok();
    let legacy = std::env::var("OHC_DATABASE_URL").ok();
    if direct.is_some() && legacy.is_some() {
        return Err("DATABASE_URL and OHC_DATABASE_URL cannot both be set".into());
    }
    direct
        .or(legacy)
        .filter(|value| !value.trim().is_empty())
        .map(DatabaseUrl::new)
        .ok_or_else(|| "DATABASE_URL is required".into())
}

pub async fn connect_from_environment() -> CommandResult<AppDatabase> {
    let url = database_url_from_environment()?;
    Ok(AppDatabase::connect(url.expose_for_connection()).await?)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DatabaseVerification {
    pub backend: super::capabilities::DatabaseBackend,
    pub migrations: u64,
    pub users: u64,
    pub products: u64,
}

pub async fn verify(database: &AppDatabase) -> CommandResult<DatabaseVerification> {
    database
        .connection()
        .execute(sea_orm::Statement::from_string(
            database.connection().get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await?;
    Ok(DatabaseVerification {
        backend: database.backend(),
        migrations: entities::schema_version::Entity::find()
            .count(database.connection())
            .await?,
        users: entities::user::Entity::find()
            .count(database.connection())
            .await?,
        products: entities::product::Entity::find()
            .count(database.connection())
            .await?,
    })
}

pub async fn migrate_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    run_legacy_postgres_migrations(&database).await?;
    migration::migrate(&database).await?;
    Ok(())
}

async fn run_legacy_postgres_migrations(database: &AppDatabase) -> CommandResult {
    if database.backend() != super::capabilities::DatabaseBackend::Postgres {
        return Ok(());
    }
    return Ok(());

}

pub async fn bootstrap_admin(database: &AppDatabase, email: &str, password: &str) -> CommandResult {
    let normalized_email = server_auth::validation::normalize_email(email)
        .map_err(|_| "ADMIN_EMAIL must be a valid email address")?;
    if password.len() < 12 {
        return Err("ADMIN_PASSWORD must contain at least 12 characters".into());
    }
    migration::migrate(database).await?;
    let tenant_id = "org-1";
    let password = password.to_owned();
    let password_hash = tokio::task::spawn_blocking(move || bcrypt::hash(password, 10)).await??;
    let now = Utc::now();
    let transaction =
        server_auth::seaorm_store::begin_tenant_transaction(database.connection(), tenant_id)
            .await
            .map_err(std::io::Error::other)?;
    let existing = entities::user::Entity::find()
        .filter(entities::user::Column::TenantId.eq(tenant_id))
        .all(&transaction)
        .await?
        .into_iter()
        .find(|user| {
            server_auth::validation::normalize_email(&user.email)
                .is_ok_and(|candidate| candidate == normalized_email)
        });
    let user_id = existing
        .as_ref()
        .map(|user| user.id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    match server_auth::seaorm_store::entities::identity_email_claim::Entity::find_by_id(
        &normalized_email,
    )
    .one(&transaction)
    .await?
    {
        Some(claim) if claim.user_id != user_id => {
            return Err(std::io::Error::other(format!(
                "identity email collision for normalized email {normalized_email}"
            ))
            .into());
        }
        Some(_) => {}
        None => {
            server_auth::seaorm_store::entities::identity_email_claim::ActiveModel {
                normalized_email: Set(normalized_email.clone()),
                user_id: Set(user_id.clone()),
                claimed_at: Set(now),
            }
            .insert(&transaction)
            .await?;
        }
    }

    if let Some(existing) = existing {
        let mut admin: entities::user::ActiveModel = existing.into();
        admin.username = Set(normalized_email.clone());
        admin.email = Set(normalized_email);
        admin.password_hash = Set(password_hash);
        admin.active = Set(true);
        admin.updated_at = Set(now);
        admin.update(&transaction).await?;
        server_auth::seaorm_store::replace_user_roles(
            &transaction,
            &user_id,
            tenant_id,
            &["ADMIN".to_string()],
        )
        .await
        .map_err(std::io::Error::other)?;
        transaction.commit().await?;
        return Ok(());
    }

    entities::user::ActiveModel {
        id: Set(user_id.clone()),
        username: Set(normalized_email.clone()),
        email: Set(normalized_email),
        password_hash: Set(password_hash),
        active: Set(true),
        tenant_id: Set(tenant_id.to_owned()),
        oidc_subject: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(&transaction)
    .await?;
    server_auth::seaorm_store::replace_user_roles(
        &transaction,
        &user_id,
        tenant_id,
        &["ADMIN".to_string()],
    )
    .await
    .map_err(std::io::Error::other)?;
    transaction.commit().await?;
    Ok(())
}

pub async fn bootstrap_admin_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    run_legacy_postgres_migrations(&database).await?;
    let email = std::env::var("ADMIN_EMAIL")?;
    let password = std::env::var("ADMIN_PASSWORD")?;
    bootstrap_admin(&database, &email, &password).await
}

pub async fn verify_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    let verification = verify(&database).await?;
    println!(
        "backend={:?} migrations={} users={} products={}",
        verification.backend, verification.migrations, verification.users, verification.products
    );
    Ok(())
}
