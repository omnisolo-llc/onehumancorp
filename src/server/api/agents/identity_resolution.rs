use sqlx::{PgPool, SqlitePool};
use crate::domain::repository::models::{Customer, CustomerIdentity};
use uuid::Uuid;

pub async fn resolve_customer_identity(
    store: &crate::db::DbStore,
    tenant_id: &str,
    provider: &str,
    identifier: &str,
) -> Result<String, String> {
    match store {
        crate::db::DbStore::Postgres => resolve_postgres(crate::db::get_pool(), tenant_id, provider, identifier).await,
        crate::db::DbStore::Sqlite(pool) => resolve_sqlite(pool, tenant_id, provider, identifier).await,
    }
}

async fn resolve_postgres(
    pool: PgPool,
    tenant_id: &str,
    provider: &str,
    identifier: &str,
) -> Result<String, String> {
    // Check if identity alias exists
    let existing_identity = sqlx::query_as::<_, CustomerIdentity>(
        "SELECT id, tenant_id, customer_id, provider, identifier, created_at FROM customer_identities WHERE tenant_id = $1 AND provider = $2 AND identifier = $3"
    )
    .bind(tenant_id)
    .bind(provider)
    .bind(identifier)
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(identity) = existing_identity {
        return Ok(identity.customer_id);
    }

    // Try finding by explicit contact info first for a direct match if the provider is email or phone
    let maybe_customer_id: Option<String> = if provider == "email" {
        sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = $1 AND email = $2")
            .bind(tenant_id)
            .bind(identifier)
            .fetch_optional(&pool)
            .await
            .map_err(|e| e.to_string())?
    } else if provider == "phone" || provider == "whatsapp" || provider == "sms" {
        sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2")
            .bind(tenant_id)
            .bind(identifier)
            .fetch_optional(&pool)
            .await
            .map_err(|e| e.to_string())?
    } else {
        None
    };

    let customer_id = if let Some(cid) = maybe_customer_id {
        cid
    } else {
        // Create new customer
        let new_customer_id = Uuid::new_v4().to_string();
        let name = format!("New {} Customer ({})", provider, identifier); // Fallback name

        let mut email = None;
        let mut phone = None;
        if provider == "email" {
            email = Some(identifier.to_string());
        } else if provider == "phone" || provider == "whatsapp" || provider == "sms" {
            phone = Some(identifier.to_string());
        }

        sqlx::query(
            "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&new_customer_id)
        .bind(tenant_id)
        .bind(&name)
        .bind(email)
        .bind(phone)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        new_customer_id
    };

    // Create the identity link
    let identity_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO customer_identities (id, tenant_id, customer_id, provider, identifier) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&identity_id)
    .bind(tenant_id)
    .bind(&customer_id)
    .bind(provider)
    .bind(identifier)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(customer_id)
}

async fn resolve_sqlite(
    pool: &SqlitePool,
    tenant_id: &str,
    provider: &str,
    identifier: &str,
) -> Result<String, String> {
    // Check if identity alias exists
    let existing_identity = sqlx::query_as::<_, CustomerIdentity>(
        "SELECT id, tenant_id, customer_id, provider, identifier, created_at FROM customer_identities WHERE tenant_id = ? AND provider = ? AND identifier = ?"
    )
    .bind(tenant_id)
    .bind(provider)
    .bind(identifier)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(identity) = existing_identity {
        return Ok(identity.customer_id);
    }

    let maybe_customer_id: Option<String> = if provider == "email" {
        sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = ? AND email = ?")
            .bind(tenant_id)
            .bind(identifier)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?
    } else if provider == "phone" || provider == "whatsapp" || provider == "sms" {
        sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = ? AND phone = ?")
            .bind(tenant_id)
            .bind(identifier)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?
    } else {
        None
    };

    let customer_id = if let Some(cid) = maybe_customer_id {
        cid
    } else {
        let new_customer_id = Uuid::new_v4().to_string();
        let name = format!("New {} Customer ({})", provider, identifier);

        let mut email = None;
        let mut phone = None;
        if provider == "email" {
            email = Some(identifier.to_string());
        } else if provider == "phone" || provider == "whatsapp" || provider == "sms" {
            phone = Some(identifier.to_string());
        }

        sqlx::query(
            "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&new_customer_id)
        .bind(tenant_id)
        .bind(&name)
        .bind(email)
        .bind(phone)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        new_customer_id
    };

    let identity_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO customer_identities (id, tenant_id, customer_id, provider, identifier) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&identity_id)
    .bind(tenant_id)
    .bind(&customer_id)
    .bind(provider)
    .bind(identifier)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(customer_id)
}
