use sqlx::{Pool, Postgres, SqlitePool};
use uuid::Uuid;
use crate::db::DbStore;

pub struct IdentityResolver {
    store: DbStore,
    pool: Pool<Postgres>,
}

impl IdentityResolver {
    pub fn new(store: DbStore, pool: Pool<Postgres>) -> Self {
        Self { store, pool }
    }

    pub async fn resolve_identity(
        &self,
        tenant_id: &str,
        provider: &str,
        provider_id: &str,
        email: Option<&str>,
        phone: Option<&str>,
        name: Option<&str>,
    ) -> Result<String, String> {
        match &self.store {
            DbStore::Postgres => {
                self.resolve_identity_pg(tenant_id, provider, provider_id, email, phone, name).await
            }
            DbStore::Sqlite(sqlite_pool) => {
                self.resolve_identity_sqlite(sqlite_pool, tenant_id, provider, provider_id, email, phone, name).await
            }
        }
    }

    async fn resolve_identity_pg(
        &self,
        tenant_id: &str,
        provider: &str,
        provider_id: &str,
        email: Option<&str>,
        phone: Option<&str>,
        name: Option<&str>,
    ) -> Result<String, String> {
        let pool = &self.pool;

        if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
            "SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND provider = $2 AND provider_id = $3 LIMIT 1",
        )
        .bind(tenant_id)
        .bind(provider)
        .bind(provider_id)
        .fetch_optional(pool)
        .await
        {
            return Ok(customer_id);
        }

        if let Some(email_str) = email {
            if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
                "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1",
            )
            .bind(tenant_id)
            .bind(email_str)
            .fetch_optional(pool)
            .await
            {
                let _ = self.link_identity_pg(pool, tenant_id, &customer_id, provider, provider_id).await;
                return Ok(customer_id);
            }
        }

        if let Some(phone_str) = phone {
            if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
                "SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2 LIMIT 1",
            )
            .bind(tenant_id)
            .bind(phone_str)
            .fetch_optional(pool)
            .await
            {
                let _ = self.link_identity_pg(pool, tenant_id, &customer_id, provider, provider_id).await;
                return Ok(customer_id);
            }
        }

        let new_customer_id = Uuid::new_v4().to_string();
        let display_name = name.unwrap_or(provider_id);

        let res = match self.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(&new_customer_id)
                .bind(tenant_id)
                .bind(display_name)
                .bind(email)
                .bind(phone)
                .execute(pool)
                .await
            }
            DbStore::Sqlite(_) => {
                sqlx::query(
                    "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(&new_customer_id)
                .bind(tenant_id)
                .bind(display_name)
                .bind(email)
                .bind(phone)
                .execute(pool)
                .await
            }
        };

        if let Err(e) = res {
            tracing::error!("Failed to create new customer: {}", e);
            return Err("Failed to create customer".to_string());
        }

        let _ = self.link_identity_pg(pool, tenant_id, &new_customer_id, provider, provider_id).await;
        Ok(new_customer_id)
    }

    async fn link_identity_pg(
        &self,
        pool: &Pool<Postgres>,
        tenant_id: &str,
        customer_id: &str,
        provider: &str,
        provider_id: &str,
    ) -> Result<(), String> {
        let id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query(
            "INSERT INTO customer_identities (id, tenant_id, customer_id, provider, provider_id) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(provider)
        .bind(provider_id)
        .execute(pool)
        .await
        {
            tracing::error!("Failed to link identity: {}", e);
            return Err("Failed to link identity".to_string());
        }
        Ok(())
    }

    async fn resolve_identity_sqlite(
        &self,
        sqlite_pool: &SqlitePool,
        tenant_id: &str,
        provider: &str,
        provider_id: &str,
        email: Option<&str>,
        phone: Option<&str>,
        name: Option<&str>,
    ) -> Result<String, String> {
        if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
            "SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND provider = ? AND provider_id = ? LIMIT 1",
        )
        .bind(tenant_id)
        .bind(provider)
        .bind(provider_id)
        .fetch_optional(sqlite_pool)
        .await
        {
            return Ok(customer_id);
        }

        if let Some(email_str) = email {
            if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
                "SELECT id FROM customers WHERE tenant_id = ? AND email = ? LIMIT 1",
            )
            .bind(tenant_id)
            .bind(email_str)
            .fetch_optional(sqlite_pool)
            .await
            {
                let _ = self.link_identity_sqlite(sqlite_pool, tenant_id, &customer_id, provider, provider_id).await;
                return Ok(customer_id);
            }
        }

        if let Some(phone_str) = phone {
            if let Ok(Some((customer_id,))) = sqlx::query_as::<_, (String,)>(
                "SELECT id FROM customers WHERE tenant_id = ? AND phone = ? LIMIT 1",
            )
            .bind(tenant_id)
            .bind(phone_str)
            .fetch_optional(sqlite_pool)
            .await
            {
                let _ = self.link_identity_sqlite(sqlite_pool, tenant_id, &customer_id, provider, provider_id).await;
                return Ok(customer_id);
            }
        }

        let new_customer_id = Uuid::new_v4().to_string();
        let display_name = name.unwrap_or(provider_id);

        if let Err(e) = sqlx::query(
            "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&new_customer_id)
        .bind(tenant_id)
        .bind(display_name)
        .bind(email)
        .bind(phone)
        .execute(sqlite_pool)
        .await
        {
            tracing::error!("Failed to create new customer: {}", e);
            return Err("Failed to create customer".to_string());
        }

        let _ = self.link_identity_sqlite(sqlite_pool, tenant_id, &new_customer_id, provider, provider_id).await;
        Ok(new_customer_id)
    }

    async fn link_identity_sqlite(
        &self,
        sqlite_pool: &SqlitePool,
        tenant_id: &str,
        customer_id: &str,
        provider: &str,
        provider_id: &str,
    ) -> Result<(), String> {
        let id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query(
            "INSERT INTO customer_identities (id, tenant_id, customer_id, provider, provider_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(provider)
        .bind(provider_id)
        .execute(sqlite_pool)
        .await
        {
            tracing::error!("Failed to link identity: {}", e);
            return Err("Failed to link identity".to_string());
        }
        Ok(())
    }
}
