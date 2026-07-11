use sqlx::{PgPool, Row};
use tracing::{error, warn};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DailyCost {
    pub date: String,
    pub total_cost: i64,
    pub llm_cost: i64,
    pub storage_cost: i64,
    pub network_cost: i64,
    pub compute_cost: i64,
    pub email_cost: i64,
    pub api_cost: i64,
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
        if let Some(d) = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(i)) {
            let d_str = d.format("%Y-%m-%d").to_string();
            trends.insert(d_str.clone(), DailyCost {
                date: d_str,
                total_cost: 0,
                llm_cost: 0,
                storage_cost: 0,
                network_cost: 0,
                compute_cost: 0,
                email_cost: 0,
                api_cost: 0,
            });
        }
    }

    for row in rows {
        if let Some(date) = row.date {
            let date_str = date.format("%Y-%m-%d").to_string();
            if let Some(daily) = trends.get_mut(&date_str) {
                let val = row.total.unwrap_or(0.0) as i64;
                match row.metric_name.as_str() {
                    "ohc_llm_cost_total_cents" => daily.llm_cost += val,
                    "ohc_storage_rw_cost" => daily.storage_cost += val,
                    "ohc_network_cost_cents" => daily.network_cost += val,
                    "ohc_compute_cost_cents" => daily.compute_cost += val,
                    "ohc_email_send_cost" => daily.email_cost += val,
                    "ohc_outbound_api_cost" | "ohc_api_call_cost" => daily.api_cost += val,
                    _ => {
                        warn!("Unknown metric encountered during aggregation: {}", row.metric_name);
                    }
                }
                daily.total_cost = daily.llm_cost + daily.storage_cost + daily.network_cost + daily.compute_cost + daily.email_cost + daily.api_cost;
            }
        } else {
            warn!("TelemetryRow missing date for metric: {}", row.metric_name);
        }
    }

    let mut sorted: Vec<DailyCost> = trends.into_values().collect();
    sorted.sort_by(|a, b| a.date.cmp(&b.date));
    sorted
}

pub async fn aggregate_daily_costs(pool: &PgPool, tenant_id: &str) -> Vec<DailyCost> {
    // Optimized Query: Cast labels_json to jsonb explicitly if it isn't already,
    // and use the ->> operator for faster execution than json_extract_path_text.
    // Ensure we handle potential errors robustly instead of silently failing.
    let raw_rows_result = sqlx::query(
        r#"
        SELECT
            DATE(timestamp) as date,
            metric_name,
            SUM(value)::FLOAT8 as total
        FROM telemetry_buffer
        WHERE tenant_id = $1
          AND metric_name IN ('ohc_llm_cost_total_cents', 'ohc_storage_rw_cost', 'ohc_network_cost_cents', 'ohc_compute_cost_cents', 'ohc_email_send_cost', 'ohc_outbound_api_cost', 'ohc_api_call_cost')
          AND timestamp >= CURRENT_DATE - INTERVAL '6 days'
        GROUP BY DATE(timestamp), metric_name
        ORDER BY DATE(timestamp) ASC
        "#
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await;

    let raw_rows = match raw_rows_result {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to fetch daily costs from database for tenant {}: {}", tenant_id, e);
            return process_telemetry_rows(vec![]); // Return empty zero-filled 7 days on error
        }
    };

    let rows: Vec<TelemetryRow> = raw_rows.into_iter().map(|r| TelemetryRow {
        date: r.try_get::<Option<chrono::NaiveDate>, _>("date").unwrap_or_else(|e| {
            warn!("Failed to parse date from query result: {}", e);
            None
        }),
        metric_name: r.try_get::<String, _>("metric_name").unwrap_or_else(|e| {
            warn!("Failed to parse metric_name from query result: {}", e);
            String::new()
        }),
        total: r.try_get::<Option<f64>, _>("total").unwrap_or_else(|e| {
            warn!("Failed to parse total from query result: {}", e);
            None
        }),
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
                metric_name: "ohc_llm_cost_total_cents".to_string(),
                total: Some(500.0),
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_compute_cost_cents".to_string(),
                total: Some(200.0),
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_email_send_cost".to_string(),
                total: Some(50.0),
            },
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_api_call_cost".to_string(),
                total: Some(100.0),
            },
        ];
        let res = process_telemetry_rows(rows);
        assert_eq!(res.len(), 7);
        let today_str = today.format("%Y-%m-%d").to_string();
        let today_data = res.iter().find(|r| r.date == today_str).expect("failed to unwrap");
        assert_eq!(today_data.llm_cost, 500);
        assert_eq!(today_data.compute_cost, 200);
        assert_eq!(today_data.email_cost, 50);
        assert_eq!(today_data.api_cost, 100);
        assert_eq!(today_data.total_cost, 850);
    }

    #[test]
    fn test_process_storage_and_network() {
        let today = chrono::Utc::now().date_naive();
        let rows = vec![
            TelemetryRow {
                date: Some(today),
                metric_name: "ohc_storage_rw_cost".to_string(),
                total: Some(50.0), // Already stored as cents
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
        let today_data = res.iter().find(|r| r.date == today_str).expect("failed to unwrap");

        assert_eq!(today_data.storage_cost, 50);
        assert_eq!(today_data.network_cost, 150);
        assert_eq!(today_data.llm_cost, 0);
        assert_eq!(today_data.compute_cost, 0);
        assert_eq!(today_data.total_cost, 200);
    }

    #[test]
    fn test_process_missing_date() {
        let rows = vec![
            TelemetryRow {
                date: None,
                metric_name: "ohc_llm_cost_total_cents".to_string(),
                total: Some(500.0),
            },
        ];
        let res = process_telemetry_rows(rows);

        // Ensure no cost was added to any date since the date was missing
        for item in res {
            assert_eq!(item.total_cost, 0);
        }
    }

    #[test]
    fn test_process_agent_cost_rows() {
        let rows = vec![
            AgentCostRawRow {
                agent_id: Some("agent1".to_string()),
                total: Some(100.0),
            },
            AgentCostRawRow {
                agent_id: None,
                total: Some(200.0),
            },
            AgentCostRawRow {
                agent_id: Some("agent3".to_string()),
                total: Some(0.0), // Should be filtered out
            },
            AgentCostRawRow {
                agent_id: Some("agent4".to_string()),
                total: None, // Should be filtered out
            },
        ];
        let res = process_agent_cost_rows(rows);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].agent_id, "agent1");
        assert_eq!(res[0].cost_cents, 100);
        assert_eq!(res[1].agent_id, "unknown");
        assert_eq!(res[1].cost_cents, 200);
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AgentCostRow {
    pub agent_id: String,
    pub cost_cents: i64,
}

pub struct AgentCostRawRow {
    pub agent_id: Option<String>,
    pub total: Option<f64>,
}

pub fn process_agent_cost_rows(rows: Vec<AgentCostRawRow>) -> Vec<AgentCostRow> {
    rows.into_iter().map(|r| AgentCostRow {
        agent_id: r.agent_id.unwrap_or_else(|| "unknown".to_string()),
        cost_cents: r.total.unwrap_or(0.0) as i64,
    })
    .filter(|r| r.cost_cents > 0)
    .collect()
}

pub async fn aggregate_agent_costs(pool: &PgPool, tenant_id: &str) -> Vec<AgentCostRow> {
    let raw_rows_result = sqlx::query(
        r#"
        SELECT
            COALESCE((labels_json::jsonb)->>'agent_id', 'unknown') as agent_id,
            SUM(value)::FLOAT8 as total
        FROM telemetry_buffer
        WHERE tenant_id = $1
          AND metric_name = 'ohc_llm_cost_total_cents'
          AND timestamp >= CURRENT_DATE - INTERVAL '30 days'
          AND labels_json IS NOT NULL
        GROUP BY COALESCE((labels_json::jsonb)->>'agent_id', 'unknown')
        ORDER BY total DESC
        "#
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await;

    let raw_rows = match raw_rows_result {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to fetch agent costs from database for tenant {}: {}", tenant_id, e);
            return vec![];
        }
    };

    let processed_rows = raw_rows.into_iter().map(|r| AgentCostRawRow {
        agent_id: r.try_get::<Option<String>, _>("agent_id").unwrap_or(None),
        total: r.try_get::<Option<f64>, _>("total").unwrap_or(None),
    }).collect();

    process_agent_cost_rows(processed_rows)
}
