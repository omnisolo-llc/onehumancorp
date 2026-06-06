use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DailyCost {
    pub date: String,
    pub total_cost: i64,
    pub llm_cost: i64,
    pub storage_cost: i64,
    pub network_cost: i64,
    pub compute_cost: i64,
}

pub struct TelemetryRow {
    pub date: Option<chrono::NaiveDate>,
    pub metric_name: String,
    pub total: Option<f64>,
}

pub fn process_telemetry_rows(rows: Vec<TelemetryRow>) -> Vec<DailyCost> {
    let mut trends = std::collections::HashMap::new();

    // Fill in last 7 days with zeros
    for i in 0..7 {
        let d = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(i)).unwrap();
        let d_str = d.format("%Y-%m-%d").to_string();
        trends.insert(d_str.clone(), DailyCost {
            date: d_str,
            total_cost: 0,
            llm_cost: 0,
            storage_cost: 0,
            network_cost: 0,
            compute_cost: 0,
        });
    }

    for row in rows {
        if let Some(date) = row.date {
            let date_str = date.format("%Y-%m-%d").to_string();
            if let Some(daily) = trends.get_mut(&date_str) {
                let val = row.total.unwrap_or(0.0) as i64;
                match row.metric_name.as_str() {
                    "ohc_mission_cost_cents" => daily.llm_cost += val,
                    "ohc_storage_rw_cost" => daily.storage_cost += (val as f64 * 0.00000001).round() as i64,
                    "ohc_network_cost_cents" => daily.network_cost += val,
                    "ohc_compute_cost_cents" => daily.compute_cost += val,
                    _ => {}
                }
                daily.total_cost = daily.llm_cost + daily.storage_cost + daily.network_cost + daily.compute_cost;
            }
        }
    }

    let mut sorted: Vec<DailyCost> = trends.into_values().collect();
    sorted.sort_by(|a, b| a.date.cmp(&b.date));
    sorted
}

pub async fn aggregate_daily_costs(pool: &PgPool, tenant_id: &str) -> Vec<DailyCost> {
    let raw_rows = sqlx::query(
        r#"
        SELECT
            DATE(timestamp) as date,
            metric_name,
            SUM(value)::FLOAT8 as total
        FROM telemetry_buffer
        WHERE json_extract_path_text(labels_json::json, 'tenant_id') = $1
          AND metric_name IN ('ohc_mission_cost_cents', 'ohc_storage_rw_cost', 'ohc_network_cost_cents', 'ohc_compute_cost_cents')
          AND timestamp >= CURRENT_DATE - INTERVAL '6 days'
        GROUP BY DATE(timestamp), metric_name
        ORDER BY DATE(timestamp) ASC
        "#
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let rows = raw_rows.into_iter().map(|r| TelemetryRow {
        date: r.try_get::<Option<chrono::NaiveDate>, _>("date").unwrap_or(None),
        metric_name: r.try_get::<String, _>("metric_name").unwrap_or_default(),
        total: r.try_get::<Option<f64>, _>("total").unwrap_or(None),
    }).collect();

    process_telemetry_rows(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_empty() {
        let rows = vec![];
        let res = process_telemetry_rows(rows);
        assert_eq!(res.len(), 7);
        for item in res {
            assert_eq!(item.total_cost, 0);
        }
    }

    #[test]
    fn test_process_with_data() {
        let today = chrono::Utc::now().date_naive();
        let rows = vec![
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_mission_cost_cents".to_string(),
                total: Some(500.0),
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_compute_cost_cents".to_string(),
                total: Some(200.0),
            },
        ];
        let res = process_telemetry_rows(rows);
        assert_eq!(res.len(), 7);
        let today_str = today.format("%Y-%m-%d").to_string();
        let today_data = res.iter().find(|r| r.date == today_str).unwrap();
        assert_eq!(today_data.llm_cost, 500);
        assert_eq!(today_data.compute_cost, 200);
        assert_eq!(today_data.total_cost, 700);
    }

    #[test]
    fn test_process_storage_and_network() {
        let today = chrono::Utc::now().date_naive();
        let rows = vec![
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_storage_rw_cost".to_string(),
                total: Some(100000000.0), // Should become 1 after mult with 0.00000001
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_network_cost_cents".to_string(),
                total: Some(150.0), // Should be 150
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "unrelated_metric".to_string(),
                total: Some(999.0),
            },
        ];
        let res = process_telemetry_rows(rows);
        let today_str = today.format("%Y-%m-%d").to_string();
        let today_data = res.iter().find(|r| r.date == today_str).unwrap();

        assert_eq!(today_data.storage_cost, 1);
        assert_eq!(today_data.network_cost, 150);
        assert_eq!(today_data.llm_cost, 0);
        assert_eq!(today_data.compute_cost, 0);
        assert_eq!(today_data.total_cost, 151);
    }

    #[test]
    fn test_process_missing_date() {
        let rows = vec![
            TelemetryRow {
                date: None,
                metric_name: "ohc_mission_cost_cents".to_string(),
                total: Some(500.0),
            },
        ];
        let res = process_telemetry_rows(rows);

        // Ensure no cost was added to any date since the date was missing
        for item in res {
            assert_eq!(item.total_cost, 0);
        }
    }
}
