use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::repository::models::Customer;

pub struct IdentityResolutionEngine {
    pool: PgPool,
}

impl IdentityResolutionEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn resolve_customer(&self, tenant_id: &str, identifier: &str, source: &str) -> Result<Customer, sqlx::Error> {
        info!("Resolving identity for identifier: {} from source: {}", identifier, source);

        let mut query = sqlx::QueryBuilder::new("SELECT * FROM customers WHERE tenant_id = ");
        query.push_bind(tenant_id);

        if source == "instagram" || source == "whatsapp" {
            // Assume we check phone or a specific metadata field for social handles
            query.push(" AND (phone = ");
            query.push_bind(identifier);
            query.push(" OR email = ");
            query.push_bind(identifier);
            query.push(" OR preferences->>'social_handle' = ");
            query.push_bind(identifier);
            query.push(")");
        } else if source == "email" {
            query.push(" AND email = ");
            query.push_bind(identifier);
        } else {
            query.push(" AND email = ");
            query.push_bind(identifier);
        }

        let existing_customer = query.build_query_as::<Customer>().fetch_optional(&self.pool).await?;

        if let Some(customer) = existing_customer {
            info!("Identity matched for customer id: {}", customer.id);
            return Ok(customer);
        }

        info!("No identity matched, creating lead for identifier: {}", identifier);
        let id = Uuid::new_v4().to_string();

        // Creating an anonymous/lead record
        let new_customer = sqlx::query_as::<_, Customer>(
            r#"
            INSERT INTO customers (id, tenant_id, name, email, phone, preferences, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(format!("Lead: {}", identifier))
        .bind(if source == "email" { Some(identifier) } else { None })
        .bind(if source == "whatsapp" { Some(identifier) } else { None })
        .bind(serde_json::json!({"social_handle": identifier}))
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(new_customer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_identity_resolution() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = PgPool::connect(&database_url).await.unwrap();
        let engine = IdentityResolutionEngine::new(pool.clone());
        let tenant_id = "test-tenant";
        let handle = "sarah_bakes";

        let customer = engine.resolve_customer(tenant_id, handle, "instagram").await.unwrap();
        assert_eq!(customer.tenant_id, tenant_id);

        // Next resolution should fetch the same customer
        let customer_again = engine.resolve_customer(tenant_id, handle, "instagram").await.unwrap();
        assert_eq!(customer.id, customer_again.id);
    }
}
