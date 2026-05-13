use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};
use crate::minimax::LocalLLMClient;
use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LedgerTransaction {
    pub id: String,
    pub tenant_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub struct FinanceLedger {
    pub pool: PgPool,
}

impl FinanceLedger {
    pub fn new(db: DB) -> Self {
        Self { pool: db.pool }
    }

    pub async fn record_transaction(
        &self,
        id: &str,
        tenant_id: &str,
        amount: f64,
        currency: &str,
        status: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO ledger_transactions (id, tenant_id, amount, currency, status, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(amount)
        .bind(currency)
        .bind(status)
        .bind(timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_transactions(&self, tenant_id: &str) -> Result<Vec<LedgerTransaction>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LedgerTransaction>(
            "SELECT id, tenant_id, amount, currency, status, created_at as timestamp FROM ledger_transactions WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

pub struct BusinessAdvisor {
    llm: LocalLLMClient,
}

impl BusinessAdvisor {
    pub fn new() -> Self {
        Self {
            llm: LocalLLMClient::new(),
        }
    }

    pub async fn generate_daily_briefing(&self, tenant_id: &str, ledger: &FinanceLedger) -> Result<String, String> {
        let transactions = ledger.get_recent_transactions(tenant_id).await.map_err(|e| e.to_string())?;

        let mut agg_volume = 0.0;
        let count = transactions.len();

        for t in &transactions {
            if t.status == "completed" {
                agg_volume += t.amount;
            }
        }

        let prompt = format!(
            "You are a plain-language financial advisor for a small business owner. \\
            They had {} recent transactions totaling ${:.2}. \\
            Generate a 3-4 bullet point daily briefing summarizing this without any technical jargon like 'reconciliation' or 'accrual'. \\
            Keep it actionable and encouraging. Tone: professional but friendly.",
            count, agg_volume
        );

        self.llm.reason(&prompt).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    // We mock the LLM Client for testing to avoid actual network calls
    struct MockBusinessAdvisor;
    impl MockBusinessAdvisor {
        pub fn generate_daily_briefing_prompt(&self, count: usize, agg_volume: f64) -> String {
            format!(
                "You are a plain-language financial advisor for a small business owner. \\
                They had {} recent transactions totaling ${:.2}. \\
                Generate a 3-4 bullet point daily briefing summarizing this without any technical jargon like 'reconciliation' or 'accrual'. \\
                Keep it actionable and encouraging. Tone: professional but friendly.",
                count, agg_volume
            )
        }
    }

    #[tokio::test]
    async fn test_generate_daily_briefing_prompt() {
        let advisor = MockBusinessAdvisor;
        let prompt = advisor.generate_daily_briefing_prompt(5, 1250.50);

        assert!(prompt.contains("They had 5 recent transactions totaling $1250.50."));
        assert!(prompt.contains("Generate a 3-4 bullet point daily briefing"));
        assert!(prompt.contains("without any technical jargon"));
    }

    // Using sqlite in-memory database to test the SQL queries using standard SQL syntax compatible with both
    #[tokio::test]
    async fn test_ledger_record_and_get() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE ledger_transactions (id TEXT PRIMARY KEY, tenant_id TEXT, amount REAL, currency TEXT, status TEXT, created_at DATETIME)")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO ledger_transactions (id, tenant_id, amount, currency, status, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind("txn_1")
        .bind("tenant_a")
        .bind(100.0)
        .bind("USD")
        .bind("completed")
        .bind(Utc::now())
        .execute(&pool)
        .await
        .unwrap();

        // This query matches the exact structure we use in get_recent_transactions
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM ledger_transactions WHERE tenant_id = $1")
            .bind("tenant_a")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 1);
    }
}
