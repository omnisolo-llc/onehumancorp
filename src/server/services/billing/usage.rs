use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UsageEvent {
    pub tenant_id: Uuid,
    pub task_id: Option<String>,
    pub attempt_id: Option<String>,
    pub provider: String,
    pub model: Option<String>,
    pub payer: String,
    pub rate_revision: Option<String>,
    pub cpu_seconds_active: f64,
    pub memory_time_provisioned: f64,
    pub sandbox_time_reserved: f64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub tool_usage_count: i64,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Default)]
pub struct UsageSummary {
    pub ohc_funded_cpu_seconds: f64,
    pub ohc_funded_input_tokens: i64,
    pub ohc_funded_output_tokens: i64,
    pub byok_cpu_seconds: f64,
    pub byok_input_tokens: i64,
    pub byok_output_tokens: i64,
}

pub struct UsageTracker {
    pool: PgPool,
}

impl UsageTracker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record_usage_event(&self, event: UsageEvent) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(
            r#"
            INSERT INTO usage_events (
                tenant_id, task_id, attempt_id, provider, model, payer, rate_revision,
                cpu_seconds_active, memory_time_provisioned, sandbox_time_reserved,
                input_tokens, output_tokens, tool_usage_count, idempotency_key
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (idempotency_key) DO NOTHING
            "#,
        )
        .bind(event.tenant_id)
        .bind(event.task_id)
        .bind(event.attempt_id)
        .bind(event.provider)
        .bind(event.model)
        .bind(event.payer)
        .bind(event.rate_revision)
        .bind(event.cpu_seconds_active)
        .bind(event.memory_time_provisioned)
        .bind(event.sandbox_time_reserved)
        .bind(event.input_tokens)
        .bind(event.output_tokens)
        .bind(event.tool_usage_count)
        .bind(event.idempotency_key)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_usage_summary(&self, tenant_id: Uuid) -> Result<UsageSummary, sqlx::Error> {
        let records = sqlx::query(
            r#"
            SELECT payer, SUM(cpu_seconds_active) as cpu_seconds, SUM(input_tokens) as input_tokens, SUM(output_tokens) as output_tokens
            FROM usage_events
            WHERE tenant_id = $1
            GROUP BY payer
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let mut summary = UsageSummary::default();
        for record in records {
            let cpu_seconds: f64 = record.try_get("cpu_seconds").unwrap_or(0.0);
            let input_tokens: i64 = record.try_get("input_tokens").unwrap_or(0);
            let output_tokens: i64 = record.try_get("output_tokens").unwrap_or(0);
            let payer: String = record.try_get("payer").unwrap_or_default();

            if payer == "OHC" {
                summary.ohc_funded_cpu_seconds += cpu_seconds;
                summary.ohc_funded_input_tokens += input_tokens;
                summary.ohc_funded_output_tokens += output_tokens;
            } else {
                summary.byok_cpu_seconds += cpu_seconds;
                summary.byok_input_tokens += input_tokens;
                summary.byok_output_tokens += output_tokens;
            }
        }
        Ok(summary)
    }
}
