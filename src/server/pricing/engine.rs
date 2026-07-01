use sqlx::PgPool;
use chrono::{Utc, DateTime, Timelike};
use crate::dynamic::{DynamicPricingEngine, PricingBounds, ContextSignals, PricingRule};
use uuid::Uuid;

pub async fn apply_dynamic_pricing(
    pool: &PgPool,
    tenant_id: &str,
    target_id: &str,
    base_price_cents: i64,
) -> i64 {
    // In this repo, many services use PgPool but might actually be running against Sqlite in standalone.
    // However, the pricing rules table and price_history table are currently defined in Postgres migrations.
    // For now, we'll implement it to be safe for both if possible, but the primary target is the Postgres backend.

    let rules_rows = match sqlx::query("SELECT rules_json, base_price_cents FROM pricing_rules WHERE tenant_id = $1 AND target_id = $2 AND is_active = TRUE")
        .bind(tenant_id)
        .bind(target_id)
        .fetch_all(pool)
        .await {
            Ok(rows) => rows,
            Err(_) => return base_price_cents,
        };

    let mut rules = Vec::new();
    let mut actual_base_price = base_price_cents;
    for row in rules_rows {
        use sqlx::Row;
        // Postgres BIGINT is i64
        actual_base_price = row.try_get::<i64, _>("base_price_cents").unwrap_or(base_price_cents);
        let json: serde_json::Value = row.try_get("rules_json").unwrap_or(serde_json::json!([]));
        if let Ok(r) = serde_json::from_value::<Vec<PricingRule>>(json) {
            rules.extend(r);
        }
    }

    if rules.is_empty() {
        return base_price_cents;
    }

    let inventory_count: i32 = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
        .bind(target_id)
        .bind(tenant_id)
        .fetch_one(pool)
        .await
        .unwrap_or(100);

    let bounds = PricingBounds {
        base_price_cents: actual_base_price,
        min_price_cents: (actual_base_price as f64 * 0.5) as i64,
        max_price_cents: (actual_base_price as f64 * 2.0) as i64,
    };

    let context = ContextSignals {
        current_time: Utc::now(),
        inventory_level: inventory_count,
        inventory_velocity_7d: 1.0,
        demand_score: 0.5,
    };

    let result = DynamicPricingEngine::calculate_price(&bounds, &rules, &context);

    if result.price_cents != base_price_cents {
        let _ = sqlx::query("INSERT INTO price_history (id, tenant_id, target_id, old_price_cents, new_price_cents, reason) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(Uuid::new_v4())
            .bind(tenant_id)
            .bind(target_id)
            .bind(base_price_cents)
            .bind(result.price_cents)
            .bind(format!("Dynamic pricing: {}", result.applied_rules.join(", ")))
            .execute(pool)
            .await;
    }

    result.price_cents
}

#[inline]
pub fn calculate_heuristic_yield(start_time: DateTime<Utc>, base_price_cents: i64) -> i64 {
    if start_time.hour() >= 17 && start_time.hour() <= 20 {
        (base_price_cents as f64 * 1.15) as i64
    } else {
        base_price_cents
    }
}

pub async fn apply_yield_management(
    pool: &PgPool,
    tenant_id: &str,
    service_id: &str,
    start_time: DateTime<Utc>,
    base_price_cents: i64,
) -> i64 {
    // 1. Try to apply formal rules if they exist
    let price = apply_dynamic_pricing(pool, tenant_id, service_id, base_price_cents).await;

    // 2. Fallback to heuristic yield management if no rules changed the price
    if price == base_price_cents {
        let surge_price = calculate_heuristic_yield(start_time, base_price_cents);
        if surge_price != base_price_cents {
            let _ = sqlx::query("INSERT INTO price_history (id, tenant_id, target_id, old_price_cents, new_price_cents, reason) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(Uuid::new_v4())
                .bind(tenant_id)
                .bind(service_id)
                .bind(base_price_cents)
                .bind(surge_price)
                .bind("Heuristic yield management: Peak hour surge")
                .execute(pool).await;

            return surge_price;
        }
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_calculate_heuristic_yield() {
        // Test peak hours (17:00 to 20:59)
        let peak_time_1 = Utc.with_ymd_and_hms(2023, 10, 25, 17, 30, 0).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(peak_time_1, 1000), 1150);

        let peak_time_2 = Utc.with_ymd_and_hms(2023, 10, 25, 20, 0, 0).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(peak_time_2, 1000), 1150);

        let peak_time_3 = Utc.with_ymd_and_hms(2023, 10, 25, 19, 59, 59).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(peak_time_3, 2000), 2300);

        // Test non-peak hours
        let non_peak_time_1 = Utc.with_ymd_and_hms(2023, 10, 25, 16, 59, 59).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(non_peak_time_1, 1000), 1000);

        let non_peak_time_2 = Utc.with_ymd_and_hms(2023, 10, 25, 21, 0, 0).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(non_peak_time_2, 1000), 1000);

        let non_peak_time_3 = Utc.with_ymd_and_hms(2023, 10, 25, 12, 0, 0).single().expect("failed to unwrap");
        assert_eq!(calculate_heuristic_yield(non_peak_time_3, 5000), 5000);
    }
}

