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

async fn connect_from_environment() -> CommandResult<AppDatabase> {
    let url = database_url_from_environment()?;
    Ok(AppDatabase::connect(url.expose_for_connection()).await?)
}

pub async fn migrate_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    migration::migrate(&database).await?;
    Ok(())
}

pub async fn bootstrap_admin(database: &AppDatabase, email: &str, password: &str) -> CommandResult {
    if !email.contains('@') {
        return Err("ADMIN_EMAIL must be a valid email address".into());
    }
    if password.len() < 12 {
        return Err("ADMIN_PASSWORD must contain at least 12 characters".into());
    }
    migration::migrate(database).await?;
    let tenant_id = "org-1";
    let existing = entities::user::Entity::find()
        .filter(entities::user::Column::TenantId.eq(tenant_id))
        .filter(entities::user::Column::Email.eq(email))
        .one(database.connection())
        .await?;
    if existing.is_some() {
        return Ok(());
    }

    let password = password.to_owned();
    let password_hash = tokio::task::spawn_blocking(move || bcrypt::hash(password, 10)).await??;
    let now = Utc::now();
    entities::user::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        username: Set(email.to_owned()),
        email: Set(email.to_owned()),
        password_hash: Set(password_hash),
        roles: Set(serde_json::json!(["ADMIN"])),
        active: Set(true),
        tenant_id: Set(tenant_id.to_owned()),
        oidc_subject: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database.connection())
    .await?;
    Ok(())
}

pub async fn bootstrap_admin_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    let email = std::env::var("ADMIN_EMAIL")?;
    let password = std::env::var("ADMIN_PASSWORD")?;
    bootstrap_admin(&database, &email, &password).await
}

pub async fn verify_from_environment() -> CommandResult {
    let database = connect_from_environment().await?;
    database
        .connection()
        .execute(sea_orm::Statement::from_string(
            database.connection().get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await?;
    let migrations = entities::schema_version::Entity::find()
        .count(database.connection())
        .await?;
    let users = entities::user::Entity::find()
        .count(database.connection())
        .await?;
    let products = entities::product::Entity::find()
        .count(database.connection())
        .await?;
    println!(
        "backend={:?} migrations={} users={} products={}",
        database.backend(),
        migrations,
        users,
        products
    );
    Ok(())
}
