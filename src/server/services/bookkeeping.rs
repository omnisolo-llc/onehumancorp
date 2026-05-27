use std::sync::Arc;
use crate::db::DB;
use crate::domain::repository::models::{LedgerEntry, FinancialInsight};
use crate::minimax::MinimaxClient;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;

pub struct BookkeepingService {
    db: Arc<DB>,
}

impl BookkeepingService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn record_entry(&self, tenant_id: &str, amount: f64, entry_type: &str, description: Option<&str>) -> Result<(), String> {
        let id = Uuid::new_v4().to_string();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO ledger_entries (id, tenant_id, amount, type, description) VALUES ($1, $2, $3, $4, $5)")
                    .bind(id)
                    .bind(tenant_id)
                    .bind(amount)
                    .bind(entry_type)
                    .bind(description)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO ledger_entries (id, tenant_id, amount, type, description) VALUES (?, ?, ?, ?, ?)")
                    .bind(id)
                    .bind(tenant_id)
                    .bind(amount)
                    .bind(entry_type)
                    .bind(description)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_latest_insight(&self, tenant_id: &str) -> Result<Option<FinancialInsight>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, FinancialInsight>("SELECT id, tenant_id, plain_text_summary, suggested_action, generated_at FROM financial_insights WHERE tenant_id = $1 ORDER BY generated_at DESC LIMIT 1")
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query_as::<_, FinancialInsight>("SELECT id, tenant_id, plain_text_summary, suggested_action, generated_at FROM financial_insights WHERE tenant_id = ? ORDER BY generated_at DESC LIMIT 1")
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn generate_insight(&self, tenant_id: &str) -> Result<FinancialInsight, String> {
        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
            return Err("MINIMAX_API_KEY not set".to_string());
        }
        let client = MinimaxClient::new(api_key);

        // Aggregate recent data
        let (total_sales, total_costs): (f64, f64) = match &self.db.store {
            crate::db::DbStore::Postgres => {
                let sales: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount), 0.0) FROM ledger_entries WHERE tenant_id = $1 AND type = 'sale'")
                    .bind(tenant_id)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let costs: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount), 0.0) FROM ledger_entries WHERE tenant_id = $1 AND type = 'cost'")
                    .bind(tenant_id)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                (sales, costs)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let sales: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount), 0.0) FROM ledger_entries WHERE tenant_id = ? AND type = 'sale'")
                    .bind(tenant_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let costs: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount), 0.0) FROM ledger_entries WHERE tenant_id = ? AND type = 'cost'")
                    .bind(tenant_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                (sales, costs)
            }
        };

        let net_profit = total_sales - total_costs;
        let margin = if total_sales > 0.0 { (net_profit / total_sales) * 100.0 } else { 0.0 };

        let prompt = format!(
            "You are a financial analyst for a small business. \
             Business Data for the period: \
             Total Sales: ${:.2} \
             Total Costs: ${:.2} \
             Net Profit: ${:.2} \
             Profit Margin: {:.1}% \
             \
             Provide a brief, warm, plain-language financial summary (2 sentences) and one suggested action for the business owner. \
             Return the result in JSON format: {{\"summary\": \"...\", \"action\": \"...\"}}",
            total_sales, total_costs, net_profit, margin
        );

        let ai_response = client.reason(&prompt).await?;

        // Find JSON block in AI response if it's not pure JSON
        let json_start = ai_response.find('{').unwrap_or(0);
        let json_end = ai_response.rfind('}').unwrap_or(ai_response.len() - 1) + 1;
        let json_str = &ai_response[json_start..json_end];

        let ai_data: serde_json::Value = serde_json::from_str(json_str).map_err(|e| format!("Failed to parse AI response: {}", e))?;

        let summary = ai_data["summary"].as_str().unwrap_or("No summary available").to_string();
        let action = ai_data["action"].as_str().map(|s| s.to_string());

        let insight = FinancialInsight {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plain_text_summary: summary,
            suggested_action: action,
            generated_at: Some(Utc::now()),
        };

        // Save insight
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO financial_insights (id, tenant_id, plain_text_summary, suggested_action, generated_at) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&insight.id)
                    .bind(&insight.tenant_id)
                    .bind(&insight.plain_text_summary)
                    .bind(&insight.suggested_action)
                    .bind(insight.generated_at)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO financial_insights (id, tenant_id, plain_text_summary, suggested_action, generated_at) VALUES (?, ?, ?, ?, ?)")
                    .bind(&insight.id)
                    .bind(&insight.tenant_id)
                    .bind(&insight.plain_text_summary)
                    .bind(&insight.suggested_action)
                    .bind(insight.generated_at)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(insight)
    }
}
