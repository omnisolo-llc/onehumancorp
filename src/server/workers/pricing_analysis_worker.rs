use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;
use sqlx::Row;

pub struct PricingAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

#[derive(Debug)]
struct PricingAnalysisTarget {
    id: String,
    title: String,
    inventory_count: i32,
    _price_cents: i64,
    _metric_count: i64,
}

impl PricingAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(3600), // Run every hour
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            loop {
                let _ = Self::run_analysis(&db).await;
                tokio::time::sleep(interval_duration).await;
            }
        });
    }

    async fn run_analysis(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let tenants: Vec<String> = match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_scalar("SELECT id FROM tenants")
                    .fetch_all(&db.pool)
                    .await?
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query_scalar("SELECT id FROM tenants")
                    .fetch_all(pool)
                    .await?
            }
        };

        for tenant_id in tenants {
            let _ = Self::analyze_tenant_pricing(db, &tenant_id).await;
        }

        Ok(())
    }

    pub async fn analyze_tenant_pricing(db: &Arc<DB>, tenant_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 1. Analyze Sales Velocity for Products
        // Products with high inventory but low sales in last 7 days
        let stagnant_products = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut conn = db.pool.acquire().await?;
                ::server_common::auth_utils::set_org_context(&mut *conn, tenant_id).await.map_err(|e| e.to_string())?;
                let rows = sqlx::query(
                    r#"
                    SELECT p.id, p.title, p.inventory_count, p.price_cents,
                           COUNT(oi.id) as sales_count
                    FROM products p
                    LEFT JOIN order_items oi ON p.id = oi.product_id AND oi.created_at > CURRENT_TIMESTAMP - INTERVAL '7 days'
                    WHERE p.tenant_id = $1 AND p.type != 'booking' AND p.inventory_count > 10
                    GROUP BY p.id, p.title, p.inventory_count, p.price_cents
                    HAVING COUNT(oi.id) < 2
                    LIMIT 5
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&mut *conn)
                .await?;
                rows.into_iter().map(|r| PricingAnalysisTarget {
                    id: r.get("id"),
                    title: r.get("title"),
                    inventory_count: r.get("inventory_count"),
                    _price_cents: r.get("price_cents"),
                    _metric_count: r.get("sales_count"),
                }).collect::<Vec<_>>()
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT p.id, p.title, p.inventory_count, p.price_cents,
                           COUNT(oi.id) as sales_count
                    FROM products p
                    LEFT JOIN order_items oi ON p.id = oi.product_id AND oi.created_at > datetime('now', '-7 days')
                    WHERE p.tenant_id = ?1 AND p.type != 'booking' AND p.inventory_count > 10
                    GROUP BY p.id, p.title, p.inventory_count, p.price_cents
                    HAVING COUNT(oi.id) < 2
                    LIMIT 5
                    "#
                )
                .bind(tenant_id)
                .fetch_all(pool)
                .await?;
                rows.into_iter().map(|r| PricingAnalysisTarget {
                    id: r.get("id"),
                    title: r.get("title"),
                    inventory_count: r.get("inventory_count"),
                    _price_cents: r.get::<i32, _>("price_cents") as i64,
                    _metric_count: r.get::<i32, _>("sales_count") as i64,
                }).collect::<Vec<_>>()
            }
        };

        for target in stagnant_products {
            let proposal = json!({
                "type": "dynamic_pricing_recommendation",
                "feature_type": "dynamic_pricing",
                "target_id": target.id,
                "recommendation": format!("'{}' has high stock ({}) but low sales. Suggest a 15% discount to clear inventory.", target.title, target.inventory_count),
                "action": "create_rule",
                "rule_config": {
                    "name": format!("Clearance: {}", target.title),
                    "type": "InventoryThreshold",
                    "config": {
                        "threshold": target.inventory_count,
                        "adjustment_percent": -15.0
                    }
                }
            });

            // Autonomously apply rule
            match &db.store {
                crate::db::DbStore::Postgres => {
                    let mut conn = db.pool.acquire().await?;
                    ::server_common::auth_utils::set_org_context(&mut *conn, tenant_id).await.map_err(|e| e.to_string())?;
                    let _ = sqlx::query("INSERT INTO pricing_rules (id, tenant_id, target_id, name, base_price_cents, is_active, rules_json) VALUES ($1, $2, $3, $4, $5, FALSE, $6) ON CONFLICT (tenant_id, target_id) DO UPDATE SET rules_json = EXCLUDED.rules_json, is_active = FALSE")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(tenant_id)
                        .bind(&target.id)
                        .bind(format!("Clearance: {}", target.title))
                        .bind(target._price_cents)
                        .bind(serde_json::json!([{ "type": "InventoryThreshold", "config": { "threshold": target.inventory_count, "adjustment_percent": -15.0 } }]))
                        .execute(&mut *conn)
                        .await;
                }
                _ => {}
            }

            Self::create_feed_item(db, tenant_id, "Pricing Agent", proposal).await?;
        }

        // 2. Analyze Booking Density for Services
        // High demand slots (consistently booked)
        let popular_services = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut conn = db.pool.acquire().await?;
                ::server_common::auth_utils::set_org_context(&mut *conn, tenant_id).await.map_err(|e| e.to_string())?;
                let rows = sqlx::query(
                    r#"
                    SELECT p.id, p.title, COUNT(b.id) as booking_count
                    FROM products p
                    JOIN bookings b ON p.id = b.product_id AND b.created_at > CURRENT_TIMESTAMP - INTERVAL '14 days'
                    WHERE p.tenant_id = $1 AND p.type = 'booking'
                    GROUP BY p.id, p.title
                    HAVING COUNT(b.id) > 5
                    LIMIT 5
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&mut *conn)
                .await?;
                rows.into_iter().map(|r| PricingAnalysisTarget {
                    id: r.get("id"),
                    title: r.get("title"),
                    inventory_count: 0,
                    _price_cents: 0,
                    _metric_count: r.get("booking_count"),
                }).collect::<Vec<_>>()
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT p.id, p.title, COUNT(b.id) as booking_count
                    FROM products p
                    JOIN bookings b ON p.id = b.product_id AND b.created_at > datetime('now', '-14 days')
                    WHERE p.tenant_id = ?1 AND p.type = 'booking'
                    GROUP BY p.id, p.title
                    HAVING COUNT(b.id) > 5
                    LIMIT 5
                    "#
                )
                .bind(tenant_id)
                .fetch_all(pool)
                .await?;
                rows.into_iter().map(|r| PricingAnalysisTarget {
                    id: r.get("id"),
                    title: r.get("title"),
                    inventory_count: 0,
                    _price_cents: 0,
                    _metric_count: r.get::<i32, _>("booking_count") as i64,
                }).collect::<Vec<_>>()
            }
        };

        for target in popular_services {
            let proposal = json!({
                "type": "yield_management_recommendation",
                "feature_type": "dynamic_pricing",
                "target_id": target.id,
                "recommendation": format!("'{}' is in high demand. Suggest a 10% premium for peak hours.", target.title),
                "action": "create_rule",
                "rule_config": {
                    "name": format!("Peak Surge: {}", target.title),
                    "type": "DemandSurge",
                    "config": {
                        "threshold_score": 0.8,
                        "adjustment_percent": 10.0
                    }
                }
            });

            // Autonomously apply rule
            match &db.store {
                crate::db::DbStore::Postgres => {
                    let mut conn = db.pool.acquire().await?;
                    ::server_common::auth_utils::set_org_context(&mut *conn, tenant_id).await.map_err(|e| e.to_string())?;
                    let _ = sqlx::query("INSERT INTO pricing_rules (id, tenant_id, target_id, name, base_price_cents, is_active, rules_json) VALUES ($1, $2, $3, $4, $5, FALSE, $6) ON CONFLICT (tenant_id, target_id) DO UPDATE SET rules_json = EXCLUDED.rules_json, is_active = FALSE")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(tenant_id)
                        .bind(&target.id)
                        .bind(format!("Peak Surge: {}", target.title))
                        .bind(target._price_cents)
                        .bind(serde_json::json!([{ "type": "DemandSurge", "config": { "threshold_score": 0.8, "adjustment_percent": 10.0 } }]))
                        .execute(&mut *conn)
                        .await;
                }
                _ => {}
            }

            Self::create_feed_item(db, tenant_id, "Yield Agent", proposal).await?;
        }

        Ok(())
    }

    async fn create_feed_item(db: &Arc<DB>, tenant_id: &str, source: &str, proposal: serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let id = Uuid::new_v4().to_string();
        match &db.store {
            crate::db::DbStore::Postgres => {
                let mut conn = db.pool.acquire().await?;
                ::server_common::auth_utils::set_org_context(&mut *conn, tenant_id).await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(id)
                .bind(tenant_id)
                .bind(source)
                .bind(json!({"type": "pricing_analysis"}))
                .bind(proposal)
                .execute(&mut *conn)
                .await?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    "#
                )
                .bind(id)
                .bind(tenant_id)
                .bind(source)
                .bind(json!({"type": "pricing_analysis"}).to_string())
                .bind(proposal.to_string())
                .execute(pool)
                .await?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_pricing_analysis_heuristics() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Setup schema for test
        sqlx::query(
            "CREATE TABLE tenants (id TEXT PRIMARY KEY, name TEXT, tier TEXT);
             CREATE TABLE products (id TEXT PRIMARY KEY, tenant_id TEXT, title TEXT, inventory_count INTEGER, price_cents INTEGER, type TEXT);
             CREATE TABLE orders (id TEXT PRIMARY KEY, tenant_id TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
             CREATE TABLE order_items (id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
             CREATE TABLE bookings (id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
             CREATE TABLE agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT, event_source TEXT, context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);"
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/test").unwrap(),
            store: crate::db::DbStore::Sqlite(sqlite_pool.clone()),
        });

        let tenant_id = "test_tenant";
        sqlx::query("INSERT INTO tenants (id, name, tier) VALUES (?, 'Test', 'free')")
            .bind(tenant_id)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        // 1. Setup a stagnant product
        let stagnant_id = "prod_stagnant";
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, price_cents, type) VALUES (?, ?, 'Old Bread', 20, 500, 'product')")
            .bind(stagnant_id)
            .bind(tenant_id)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        // 2. Setup a popular service
        let popular_id = "serv_popular";
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, price_cents, type) VALUES (?, ?, 'Tutor Session', 999, 5000, 'booking')")
            .bind(popular_id)
            .bind(tenant_id)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        for i in 0..10 {
            sqlx::query("INSERT INTO bookings (id, tenant_id, product_id) VALUES (?, ?, ?)")
                .bind(format!("b_{}", i))
                .bind(tenant_id)
                .bind(popular_id)
                .execute(&sqlite_pool)
                .await
                .unwrap();
        }

        // Run analysis
        PricingAnalysisWorker::analyze_tenant_pricing(&db, tenant_id).await.unwrap();

        // Check results in agent_feed_items
        let feed_items: Vec<serde_json::Value> = sqlx::query_scalar("SELECT proposed_action FROM agent_feed_items WHERE tenant_id = ?")
            .bind(tenant_id)
            .fetch_all(&sqlite_pool)
            .await
            .unwrap()
            .into_iter()
            .map(|s: String| serde_json::from_str(&s).unwrap())
            .collect();

        assert!(feed_items.len() >= 2);

        let has_discount = feed_items.iter().any(|f| f["type"] == "dynamic_pricing_recommendation" && f["target_id"] == stagnant_id);
        let has_surge = feed_items.iter().any(|f| f["type"] == "yield_management_recommendation" && f["target_id"] == popular_id);

        assert!(has_discount, "Should have recommended a discount for stagnant product");
        assert!(has_surge, "Should have recommended a surge for popular service");
    }
}
