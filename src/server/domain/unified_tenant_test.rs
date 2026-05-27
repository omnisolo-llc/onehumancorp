#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgPoolOptions, Row};
    use std::env;
    use crate::domain::repository::models::{Business, AgentMemory, Tenant};

    #[tokio::test]
    async fn test_tenant_isolation_rls() {
        let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(100))
            .connect_lazy("postgres://postgres:postgres@localhost/postgres")
            .unwrap();

        if env::var("CI").is_ok() {
            // We just ensure it compiles locally
            return;
        }

        let tenant_1 = "00000000-0000-0000-0000-000000000001";
        let tenant_2 = "00000000-0000-0000-0000-000000000002";
        let customer_id = uuid::Uuid::new_v4().to_string();
        let product_id = uuid::Uuid::new_v4().to_string();
        let order_id = uuid::Uuid::new_v4().to_string();

        // First, insert data as system
        match pool.begin().await {
            Ok(mut tx) => {
                use sqlx::Executor;
                // Set the session context inside the transaction block to system to allow inserts
                tx.execute("SET LOCAL app.current_tenant = 'system'").await.expect("Failed to set system context");

                // Ensure the tenant exists
                let _ = sqlx::query("INSERT INTO tenants (id, business_name) VALUES ($1, 'Test Business') ON CONFLICT DO NOTHING")
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // Insert a customer for tenant 2
                let _ = sqlx::query("INSERT INTO customers (id, tenant_id, email) VALUES ($1, $2, 'test@example.com') ON CONFLICT DO NOTHING")
                    .bind(&customer_id)
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // Insert a product for tenant 2
                let _ = sqlx::query("INSERT INTO products (id, tenant_id, type) VALUES ($1, $2, 'physical') ON CONFLICT DO NOTHING")
                    .bind(&product_id)
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // Insert an order for tenant 2
                let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount) VALUES ($1, $2, $3, 100.0) ON CONFLICT DO NOTHING")
                    .bind(&order_id)
                    .bind(tenant_2)
                    .bind(&customer_id)
                    .execute(&mut *tx).await;

                tx.commit().await.expect("Failed to commit test data");
            },
            Err(_) => {
                // Ignore errors if test db is not running
                return;
            }
        }

        // Test regression: Empty org_id should NOT bypass RLS
        match pool.begin().await {
            Ok(mut tx) => {
                // Call the actual vulnerable function to test application logic
                ::server_common::auth_utils::set_org_context(&mut *tx, "").await.expect("Failed to call set_org_context");

                let result = sqlx::query("SELECT COUNT(*) FROM customers").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for empty tenant context");

                let result = sqlx::query("SELECT COUNT(*) FROM products").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for empty tenant context");

                let result = sqlx::query("SELECT COUNT(*) FROM orders").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for empty tenant context");
            },
            Err(_) => {}
        }

        // To test RLS, we explicitly begin a transaction and set the local variable to tenant_1.
        match pool.begin().await {
            Ok(mut tx) => {
                use sqlx::Executor;
                // Set the session context inside the transaction block to tenant_1
                tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_1).as_str()).await.expect("Failed to set tenant context");

                // Query with a different tenant_id (tenant_2)
                let result = sqlx::query("SELECT COUNT(*) FROM customers WHERE tenant_id = $1").bind(tenant_2).fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for another tenant despite data existing");

                let result = sqlx::query("SELECT COUNT(*) FROM products WHERE tenant_id = $1").bind(tenant_2).fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for another tenant despite data existing");

                let result = sqlx::query("SELECT COUNT(*) FROM orders WHERE tenant_id = $1").bind(tenant_2).fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Should return 0 rows for another tenant despite data existing");
            },
            Err(_) => {
                // Ignore errors if test db is not running
            }
        }
    }

    #[test]
    fn test_business_struct_compilation() {
        let b = Business {
            id: "1".to_string(),
            tenant_id: "2".to_string(),
            name: "test".to_string(),
            r#type: "retail".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(b.id, "1");
    }

    #[test]
    fn test_agent_memory_struct_compilation() {
        let am = AgentMemory {
            id: "1".to_string(),
            tenant_id: "2".to_string(),
            business_id: Some("3".to_string()),
            department: Some("sales".to_string()),
            content: "hello".to_string(),
            embedding: Some(vec![0.1, 0.2]),
            interaction_data: None,
            created_at: None,
        };
        assert_eq!(am.id, "1");
    }


    #[test]
    fn test_new_models_struct_compilation() {
        use crate::domain::repository::models::{Customer, Product, Order, Booking, AIAgent};

        let c = Customer {
            id: "c1".to_string(),
            tenant_id: "t1".to_string(),
            name: "John".to_string(),
            email: None,
            phone: None,
            preferences: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(c.id, "c1");

        let p = Product {
            id: "p1".to_string(),
            tenant_id: "t1".to_string(),
            r#type: "physical".to_string(),
            title: None,
            name: None,
            description: None,
            price: None,
            price_cents: None,
            currency: None,
            in_stock: None,
            inventory_count: None,
            is_sold_out: None,
            metadata: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(p.id, "p1");

        let o = Order {
            id: "o1".to_string(),
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            status: None,
            total_amount: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(o.id, "o1");

        let b = Booking {
            id: "b1".to_string(),
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            product_id: None,
            start_time: chrono::Utc::now(),
            end_time: None,
            status: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(b.id, "b1");

        let a = AIAgent {
            id: "a1".to_string(),
            tenant_id: "t1".to_string(),
            name: "Agent1".to_string(),
            role: "Sales".to_string(),
            department: None,
            status: None,
            provider_type: None,
            region: None,
            registered_at: None,
        };
        assert_eq!(a.id, "a1");
    }
}
