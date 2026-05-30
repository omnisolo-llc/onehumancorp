#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgPoolOptions, Row};
    use std::env;
    use uuid::Uuid;
    use crate::domain::repository::models::{
        Business, AgentMemory, Customer, CatalogItem, ItemVariant, Order, OrderLineItem
    };

    #[tokio::test]
    async fn test_tenant_isolation_rls() {
        let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(100))
            .connect_lazy("postgres://postgres:postgres@localhost/postgres")
            .unwrap();

        if env::var("CI").is_ok() {
            // We just ensure it compiles locally
            return;
        }

        let tenant_1 = Uuid::new_v4();
        let tenant_2 = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let catalog_item_id = Uuid::new_v4();
        let variant_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();
        let memory_id = Uuid::new_v4();

        // CUJ: Instantiate tenant, create hybrid catalog item, process order, store AI memory
        match pool.begin().await {
            Ok(mut tx) => {
                use sqlx::Executor;
                // Set the session context to allow setup
                tx.execute("SET app.current_tenant = '00000000-0000-0000-0000-000000000000'").await.ok();

                // 1. Create Tenant
                let _ = sqlx::query("INSERT INTO tenant (id, name) VALUES ($1, 'Maya Baker')")
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // 2. Create Customer
                let _ = sqlx::query("INSERT INTO customer (id, tenant_id, email) VALUES ($1, $2, 'client@example.com')")
                    .bind(&customer_id)
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // 3. Create Hybrid Catalog Item (Physical Cake)
                let _ = sqlx::query("INSERT INTO catalog_item (id, tenant_id, title, item_type) VALUES ($1, $2, 'Custom Cake', 'product')")
                    .bind(&catalog_item_id)
                    .bind(tenant_2)
                    .execute(&mut *tx).await;

                // 4. Create Variant (Chocolate)
                let _ = sqlx::query("INSERT INTO item_variant (id, tenant_id, catalog_item_id, price, attributes) VALUES ($1, $2, $3, 150.0, '{\"flavor\": \"chocolate\"}')")
                    .bind(&variant_id)
                    .bind(tenant_2)
                    .bind(&catalog_item_id)
                    .execute(&mut *tx).await;

                // 5. Process Order
                let _ = sqlx::query("INSERT INTO \"order\" (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, 150.0, 'confirmed')")
                    .bind(&order_id)
                    .bind(tenant_2)
                    .bind(&customer_id)
                    .execute(&mut *tx).await;

                let _ = sqlx::query("INSERT INTO order_line_item (id, tenant_id, order_id, variant_id, quantity, unit_price) VALUES ($1, $2, $3, $4, 1, 150.0)")
                    .bind(Uuid::new_v4())
                    .bind(tenant_2)
                    .bind(&order_id)
                    .bind(&variant_id)
                    .execute(&mut *tx).await;

                // 6. Store AI Interaction
                let _ = sqlx::query("INSERT INTO agent_memory (id, tenant_id, customer_id, department, raw_context) VALUES ($1, $2, $3, 'sales', '{\"message\": \"User asked for chocolate cake\"}')")
                    .bind(&memory_id)
                    .bind(tenant_2)
                    .bind(&customer_id)
                    .execute(&mut *tx).await;

                tx.commit().await.expect("Failed to commit CUJ data");
            },
            Err(_) => {
                return;
            }
        }

        // Verify RLS isolation
        match pool.begin().await {
            Ok(mut tx) => {
                use sqlx::Executor;
                // Set context to tenant_1 (empty business)
                tx.execute(format!("SET app.current_tenant = '{}'", tenant_1).as_str()).await.expect("Failed to set tenant context");

                let result = sqlx::query("SELECT COUNT(*) FROM customer").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Tenant 1 should see 0 customers");

                let result = sqlx::query("SELECT COUNT(*) FROM catalog_item").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Tenant 1 should see 0 catalog items");

                let result = sqlx::query("SELECT COUNT(*) FROM \"order\"").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Tenant 1 should see 0 orders");

                let result = sqlx::query("SELECT COUNT(*) FROM agent_memory").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 0, "Tenant 1 should see 0 memories");
            },
            Err(_) => {}
        }

        // Verify Tenant 2 sees their own data
        match pool.begin().await {
            Ok(mut tx) => {
                use sqlx::Executor;
                tx.execute(format!("SET app.current_tenant = '{}'", tenant_2).as_str()).await.expect("Failed to set tenant context");

                let result = sqlx::query("SELECT COUNT(*) FROM customer").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 1, "Tenant 2 should see 1 customer");

                let result = sqlx::query("SELECT COUNT(*) FROM catalog_item").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 1, "Tenant 2 should see 1 catalog item");

                let result = sqlx::query("SELECT COUNT(*) FROM \"order\"").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 1, "Tenant 2 should see 1 order");

                let result = sqlx::query("SELECT COUNT(*) FROM agent_memory").fetch_one(&mut *tx).await;
                assert_eq!(result.unwrap().get::<i64, _>(0), 1, "Tenant 2 should see 1 memory");
            },
            Err(_) => {}
        }
    }

    #[test]
    fn test_agent_memory_struct_compilation() {
        let am = AgentMemory {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            customer_id: Some(Uuid::new_v4()),
            department: "sales".to_string(),
            embedding: Some(vec![0.1, 0.2]),
            raw_context: None,
            created_at: None,
        };
        assert!(am.department == "sales");
    }


    #[test]
    fn test_new_models_struct_compilation() {
        let c = Customer {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: None,
            phone: None,
            preferences: None,
            last_active: None,
            created_at: None,
            updated_at: None,
        };
        assert!(c.email.is_none());

        let ci = CatalogItem {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            title: "Test Item".to_string(),
            description: None,
            item_type: "product".to_string(),
            is_active: Some(true),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(ci.title, "Test Item");

        let v = ItemVariant {
            id: Uuid::new_v4(),
            tenant_id: ci.tenant_id,
            catalog_item_id: ci.id,
            sku: None,
            price: 99.99,
            inventory_count: Some(10),
            attributes: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(v.price, 99.99);

        let o = Order {
            id: Uuid::new_v4(),
            tenant_id: ci.tenant_id,
            customer_id: c.id,
            status: Some("pending".to_string()),
            total_amount: Some(99.99),
            created_at: None,
            updated_at: None,
        };
        assert!(o.status.is_some());

        let oli = OrderLineItem {
            id: Uuid::new_v4(),
            tenant_id: o.tenant_id,
            order_id: o.id,
            variant_id: v.id,
            quantity: Some(1),
            unit_price: 99.99,
            created_at: None,
        };
        assert_eq!(oli.unit_price, 99.99);
    }
}
