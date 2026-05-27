use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::db::DbStore;
use std::sync::Arc;
use uuid::Uuid;

pub async fn evaluate_yield(
    orchestrator: Arc<DepartmentOrchestrator>,
    tenant_id: String,
) -> Result<(), String> {
    let db = orchestrator.db();

    // 1. Evaluate Excess Inventory (Products)
    let excess_products_query = "
        SELECT id, price, min_price
        FROM products
        WHERE tenant_id = $1
          AND auto_pricing_enabled = true
          AND inventory_count > 10
          AND min_price IS NOT NULL
    ";

    let mut dropped_events = Vec::new();

    match &db.store {
        DbStore::Postgres => {
            if let Ok(rows) = sqlx::query(excess_products_query)
                .bind(&tenant_id)
                .fetch_all(&db.pool)
                .await
            {
                use sqlx::Row;
                for row in rows {
                    let product_id: String = row.get("id");
                    // Using rust's dynamic types to fetch decimals as floats to keep things simple
                    let mut price: f64 = 0.0;
                    if let Ok(sqlx::types::BigDecimal::Some) = row.try_get::<sqlx::types::BigDecimal, _>("price") {
                        // For simplicity, skip proper arbitrary-precision math if we can't easily parse
                    } else if let Ok(p) = row.try_get::<f64, _>("price") {
                        price = p;
                    }
                    let min_price: f64 = row.try_get("min_price").unwrap_or(0.0);

                    if price > 0.0 {
                        let new_price = (price * 0.8).max(min_price);

                        if new_price < price {
                            let _ = sqlx::query("UPDATE products SET price = $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(new_price)
                                .bind(&product_id)
                                .bind(&tenant_id)
                                .execute(&db.pool)
                                .await;

                            let event_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query(
                                "INSERT INTO yield_events (id, tenant_id, target_type, target_id, original_price, new_price, reason) VALUES ($1, $2, $3, $4, $5, $6, $7)"
                            )
                            .bind(&event_id)
                            .bind(&tenant_id)
                            .bind("product")
                            .bind(&product_id)
                            .bind(price)
                            .bind(new_price)
                            .bind("Excess inventory > 10")
                            .execute(&db.pool)
                            .await;

                            dropped_events.push(product_id);
                        }
                    }
                }
            }
        }
        DbStore::Sqlite(pool) => {
             // simplified logic for sqlite if used
             if let Ok(rows) = sqlx::query("SELECT id, price, min_price FROM products WHERE tenant_id = ? AND auto_pricing_enabled = true AND inventory_count > 10 AND min_price IS NOT NULL")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await
            {
                use sqlx::Row;
                for row in rows {
                    let product_id: String = row.get("id");
                    let price: f64 = row.try_get("price").unwrap_or(0.0);
                    let min_price: f64 = row.try_get("min_price").unwrap_or(0.0);

                    if price > 0.0 {
                        let new_price = (price * 0.8).max(min_price);
                        if new_price < price {
                            let _ = sqlx::query("UPDATE products SET price = ? WHERE id = ? AND tenant_id = ?")
                                .bind(new_price)
                                .bind(&product_id)
                                .bind(&tenant_id)
                                .execute(pool)
                                .await;

                            let event_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query("INSERT INTO yield_events (id, tenant_id, target_type, target_id, original_price, new_price, reason) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&event_id)
                                .bind(&tenant_id)
                                .bind("product")
                                .bind(&product_id)
                                .bind(price)
                                .bind(new_price)
                                .bind("Excess inventory > 10")
                                .execute(pool)
                                .await;

                            dropped_events.push(product_id);
                        }
                    }
                }
            }
        }
    }

    if !dropped_events.is_empty() {
        let _ = orchestrator.dispatch_event(DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.yield.price_dropped".to_string(),
            payload: serde_json::json!({ "dropped_product_ids": dropped_events }),
        }).await;
    }

    // 2. Evaluate Calendar Density
    let density_query = "
        SELECT p.id, p.price, p.max_price, COUNT(b.id) as booking_count
        FROM products p
        LEFT JOIN bookings b ON p.id = b.product_id
        WHERE p.tenant_id = $1
          AND p.auto_pricing_enabled = true
          AND p.max_price IS NOT NULL
        GROUP BY p.id, p.price, p.max_price
        HAVING COUNT(b.id) > 5
    ";

    let mut raised_events = Vec::new();

    match &db.store {
        DbStore::Postgres => {
            if let Ok(rows) = sqlx::query(density_query)
                .bind(&tenant_id)
                .fetch_all(&db.pool)
                .await
            {
                use sqlx::Row;
                for row in rows {
                    let product_id: String = row.get("id");
                    let mut price: f64 = 0.0;
                    if let Ok(p) = row.try_get::<f64, _>("price") {
                        price = p;
                    }
                    let max_price: f64 = row.try_get("max_price").unwrap_or(price);

                    if price > 0.0 {
                        let new_price = (price * 1.2).min(max_price);
                        if new_price > price {
                            let _ = sqlx::query("UPDATE products SET price = $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(new_price)
                                .bind(&product_id)
                                .bind(&tenant_id)
                                .execute(&db.pool)
                                .await;

                            let event_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query(
                                "INSERT INTO yield_events (id, tenant_id, target_type, target_id, original_price, new_price, reason) VALUES ($1, $2, $3, $4, $5, $6, $7)"
                            )
                            .bind(&event_id)
                            .bind(&tenant_id)
                            .bind("product")
                            .bind(&product_id)
                            .bind(price)
                            .bind(new_price)
                            .bind("High calendar density > 5 bookings")
                            .execute(&db.pool)
                            .await;

                            raised_events.push(product_id);
                        }
                    }
                }
            }
        }
        DbStore::Sqlite(pool) => {
             let sqlite_density_query = "
                 SELECT p.id, p.price, p.max_price, COUNT(b.id) as booking_count
                 FROM products p
                 LEFT JOIN bookings b ON p.id = b.product_id
                 WHERE p.tenant_id = ?
                   AND p.auto_pricing_enabled = true
                   AND p.max_price IS NOT NULL
                 GROUP BY p.id, p.price, p.max_price
                 HAVING COUNT(b.id) > 5
             ";
             if let Ok(rows) = sqlx::query(sqlite_density_query)
                .bind(&tenant_id)
                .fetch_all(pool)
                .await
            {
                use sqlx::Row;
                for row in rows {
                    let product_id: String = row.get("id");
                    let price: f64 = row.try_get("price").unwrap_or(0.0);
                    let max_price: f64 = row.try_get("max_price").unwrap_or(price);

                    if price > 0.0 {
                        let new_price = (price * 1.2).min(max_price);
                        if new_price > price {
                            let _ = sqlx::query("UPDATE products SET price = ? WHERE id = ? AND tenant_id = ?")
                                .bind(new_price)
                                .bind(&product_id)
                                .bind(&tenant_id)
                                .execute(pool)
                                .await;

                            let event_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query("INSERT INTO yield_events (id, tenant_id, target_type, target_id, original_price, new_price, reason) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&event_id)
                                .bind(&tenant_id)
                                .bind("product")
                                .bind(&product_id)
                                .bind(price)
                                .bind(new_price)
                                .bind("High calendar density > 5 bookings")
                                .execute(pool)
                                .await;

                            raised_events.push(product_id);
                        }
                    }
                }
            }
        }
    }

    if !raised_events.is_empty() {
        let _ = orchestrator.dispatch_event(DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.yield.price_raised".to_string(),
            payload: serde_json::json!({ "raised_product_ids": raised_events }),
        }).await;
    }

    Ok(())
}
