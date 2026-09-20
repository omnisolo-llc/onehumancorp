//! Durable usage admission and settlement, separate from lossy observability.
//! Amounts are USD micro-units, not floating-point dollars or per-event cents.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, SqlitePool};

#[derive(Clone)]
pub enum UsageLedger {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayerMode {
    ManagedApi,
    ByokApi,
    NativeSubscription,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RateCard {
    pub revision: String,
    pub input_micros_per_million: i64,
    pub output_micros_per_million: i64,
    pub cached_input_micros_per_million: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UsageScope {
    pub tenant_id: String,
    pub task_id: String,
    pub attempt_id: String,
    pub provider: String,
    pub model: String,
    pub payer: PayerMode,
    pub rate_card: Option<RateCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TokenCounts {
    /// Total input, including cached input. Cached input must not be double billed.
    pub input: i64,
    pub output: i64,
    pub cached_input: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsageReceipt {
    pub provider_request_id: String,
    pub counts: Option<TokenCounts>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsageRecord {
    pub event_id: String,
    pub state: String,
    pub scope: UsageScope,
    pub reserved_micros: i64,
    pub charged_micros: Option<i64>,
    pub provider_cost_micros: Option<i64>,
    pub receipt: Option<UsageReceipt>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AccountSummary {
    pub limit_micros: i64,
    pub spent_micros: i64,
    pub reserved_micros: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Admitted,
    Duplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerError {
    Invalid,
    Unconfigured,
    Limit,
    Conflict,
    State,
    Database,
}
impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Invalid => "invalid usage record",
            Self::Unconfigured => "usage budget is not configured",
            Self::Limit => "usage budget exhausted",
            Self::Conflict => "usage identity conflicts with prior work",
            Self::State => "usage requires reconciliation",
            Self::Database => "usage ledger is unavailable",
        })
    }
}
impl std::error::Error for LedgerError {}
impl From<sqlx::Error> for LedgerError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

fn identifier(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 255 && !value.chars().any(char::is_control)
}
fn digest<T: Serialize>(value: &T) -> Result<String, LedgerError> {
    Ok(format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).map_err(|_| LedgerError::Invalid)?)
    ))
}
impl RateCard {
    pub fn cost(&self, counts: &TokenCounts) -> Result<i64, LedgerError> {
        if !identifier(&self.revision)
            || [
                self.input_micros_per_million,
                self.output_micros_per_million,
                self.cached_input_micros_per_million,
            ]
            .iter()
            .any(|n| *n < 0 || *n > 1_000_000_000_000)
            || counts.input < 0
            || counts.output < 0
            || counts.cached_input < 0
            || counts.cached_input > counts.input
        {
            return Err(LedgerError::Invalid);
        }
        let numerator = i128::from(counts.input - counts.cached_input)
            * i128::from(self.input_micros_per_million)
            + i128::from(counts.output) * i128::from(self.output_micros_per_million)
            + i128::from(counts.cached_input) * i128::from(self.cached_input_micros_per_million);
        i64::try_from((numerator + 999_999) / 1_000_000).map_err(|_| LedgerError::Invalid)
    }
}
impl UsageScope {
    fn validate(&self) -> Result<(), LedgerError> {
        if [
            &self.tenant_id,
            &self.task_id,
            &self.attempt_id,
            &self.provider,
            &self.model,
        ]
        .iter()
        .any(|value| !identifier(value))
        {
            return Err(LedgerError::Invalid);
        }
        if self.payer == PayerMode::ManagedApi && self.rate_card.is_none() {
            return Err(LedgerError::Unconfigured);
        }
        if let Some(rate) = &self.rate_card {
            rate.cost(&TokenCounts {
                input: 0,
                output: 0,
                cached_input: 0,
            })?;
        }
        Ok(())
    }
}

// Expand against concrete drivers: no Any-driver ambiguity in money arithmetic.
macro_rules! transaction {
    ($ledger:expr, $tenant:expr, $tx:ident, $body:block) => {{
        match $ledger {
            UsageLedger::Postgres(pool) => {
                let mut $tx = pool.begin().await?;
                sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind($tenant)
                    .execute(&mut *$tx)
                    .await?;
                $body
            }
            UsageLedger::Sqlite(pool) => {
                let mut $tx = pool.begin().await?;
                $body
            }
        }
    }};
}

impl UsageLedger {
    /// Deployment migration hook. Never loads credentials from a request payload.
    pub async fn initialize(&self) -> Result<(), LedgerError> {
        const SCHEMA: [&str; 3] = [
            "CREATE TABLE IF NOT EXISTS ohc_usage_accounts (tenant_id TEXT PRIMARY KEY, limit_micros BIGINT NOT NULL CHECK(limit_micros >= 0), spent_micros BIGINT NOT NULL DEFAULT 0 CHECK(spent_micros >= 0), reserved_micros BIGINT NOT NULL DEFAULT 0 CHECK(reserved_micros >= 0))",
            "CREATE TABLE IF NOT EXISTS ohc_usage_records (tenant_id TEXT NOT NULL, event_id TEXT NOT NULL, request_digest TEXT NOT NULL, scope_json TEXT NOT NULL, state TEXT NOT NULL, reserved_micros BIGINT NOT NULL CHECK(reserved_micros >= 0), charged_micros BIGINT, provider_cost_micros BIGINT, receipt_json TEXT, receipt_digest TEXT, created_at TEXT NOT NULL DEFAULT (CAST(CURRENT_TIMESTAMP AS TEXT)), PRIMARY KEY(tenant_id,event_id), FOREIGN KEY(tenant_id) REFERENCES ohc_usage_accounts(tenant_id))",
            "CREATE TABLE IF NOT EXISTS ohc_usage_receipts (tenant_id TEXT NOT NULL, provider TEXT NOT NULL, provider_request_id TEXT NOT NULL, event_id TEXT NOT NULL, PRIMARY KEY(tenant_id,provider,provider_request_id), FOREIGN KEY(tenant_id,event_id) REFERENCES ohc_usage_records(tenant_id,event_id))",
        ];
        match self {
            Self::Postgres(pool) => {
                let mut tx = pool.begin().await?;
                // Serialize schema installation independently of customer transactions.
                sqlx::query("SELECT pg_advisory_xact_lock(734562191)")
                    .execute(&mut *tx)
                    .await?;
                for ddl in SCHEMA {
                    sqlx::query(ddl).execute(&mut *tx).await?;
                }
                for table in [
                    "ohc_usage_accounts",
                    "ohc_usage_records",
                    "ohc_usage_receipts",
                ] {
                    sqlx::query(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY"))
                        .execute(&mut *tx)
                        .await?;
                    sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
                        .execute(&mut *tx)
                        .await?;
                    sqlx::query(&format!("DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname=current_schema() AND tablename='{table}' AND policyname='ohc_usage_tenant') THEN CREATE POLICY ohc_usage_tenant ON {table} USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true)); END IF; END $$")).execute(&mut *tx).await?;
                }
                tx.commit().await?;
            }
            Self::Sqlite(pool) => {
                for ddl in SCHEMA {
                    sqlx::query(ddl).execute(pool).await?;
                }
            }
        }
        Ok(())
    }

    /// Set a spending authorization, NOT credit a payment balance. Caller must
    /// authenticate the owner/admin. Lowering below outstanding exposure fails.
    pub async fn set_limit(&self, tenant: &str, limit: i64) -> Result<(), LedgerError> {
        if !identifier(tenant) || limit < 0 {
            return Err(LedgerError::Invalid);
        }
        transaction!(self, tenant, tx, {
            let changed = sqlx::query("INSERT INTO ohc_usage_accounts(tenant_id,limit_micros) VALUES($1,$2) ON CONFLICT(tenant_id) DO UPDATE SET limit_micros=excluded.limit_micros WHERE ohc_usage_accounts.spent_micros <= excluded.limit_micros AND ohc_usage_accounts.reserved_micros <= excluded.limit_micros - ohc_usage_accounts.spent_micros")
                .bind(tenant).bind(limit).execute(&mut *tx).await?.rows_affected();
            if changed != 1 {
                return Err(LedgerError::Limit);
            }
            tx.commit().await?;
            Ok(())
        })
    }

    pub async fn reserve(
        &self,
        scope: &UsageScope,
        event: &str,
        request_digest: &str,
        maximum: i64,
    ) -> Result<Admission, LedgerError> {
        scope.validate()?;
        if !identifier(event)
            || request_digest.len() != 64
            || !request_digest.bytes().all(|v| v.is_ascii_hexdigit())
            || maximum < 0
            || (scope.payer != PayerMode::ManagedApi && maximum != 0)
        {
            return Err(LedgerError::Invalid);
        }
        let context = serde_json::to_string(scope).map_err(|_| LedgerError::Invalid)?;
        transaction!(self, &scope.tenant_id, tx, {
            // First statement is a write, avoiding SQLite read-to-write upgrades.
            let inserted = sqlx::query("INSERT INTO ohc_usage_records(tenant_id,event_id,request_digest,scope_json,state,reserved_micros) SELECT $1,$2,$3,$4,'reserved',$5 WHERE EXISTS(SELECT 1 FROM ohc_usage_accounts WHERE tenant_id=$1) ON CONFLICT(tenant_id,event_id) DO NOTHING")
                .bind(&scope.tenant_id).bind(event).bind(request_digest).bind(&context).bind(maximum).execute(&mut *tx).await?.rows_affected();
            if inserted == 0 {
                let old: Option<(String,String,i64)> = sqlx::query_as("SELECT request_digest,scope_json,reserved_micros FROM ohc_usage_records WHERE tenant_id=$1 AND event_id=$2")
                    .bind(&scope.tenant_id).bind(event).fetch_optional(&mut *tx).await?;
                return match old {
                    Some((old_digest, old_context, old_maximum))
                        if old_digest == request_digest
                            && old_context == context
                            && old_maximum == maximum =>
                    {
                        Ok(Admission::Duplicate)
                    }
                    Some(_) => Err(LedgerError::Conflict),
                    None => Err(LedgerError::Unconfigured),
                };
            }
            let admitted = sqlx::query("UPDATE ohc_usage_accounts SET reserved_micros=reserved_micros+$2 WHERE tenant_id=$1 AND spent_micros <= limit_micros AND reserved_micros <= limit_micros-spent_micros AND $2 <= limit_micros-spent_micros-reserved_micros")
                .bind(&scope.tenant_id).bind(maximum).execute(&mut *tx).await?.rows_affected();
            if admitted != 1 {
                return Err(LedgerError::Limit);
            }
            tx.commit().await?;
            Ok(Admission::Admitted)
        })
    }

    pub async fn dispatched(&self, tenant: &str, event: &str) -> Result<(), LedgerError> {
        if !identifier(tenant) || !identifier(event) {
            return Err(LedgerError::Invalid);
        }
        transaction!(self, tenant, tx, {
            let changed = sqlx::query("UPDATE ohc_usage_records SET state='in_flight' WHERE tenant_id=$1 AND event_id=$2 AND state='reserved'")
                .bind(tenant).bind(event).execute(&mut *tx).await?.rows_affected();
            if changed != 1 {
                return Err(LedgerError::State);
            }
            tx.commit().await?;
            Ok(())
        })
    }

    pub async fn cancel_before_dispatch(
        &self,
        tenant: &str,
        event: &str,
    ) -> Result<(), LedgerError> {
        transaction!(self, tenant, tx, {
            let amount: Option<(i64,)> = sqlx::query_as("UPDATE ohc_usage_records SET state='cancelled',charged_micros=0 WHERE tenant_id=$1 AND event_id=$2 AND state='reserved' RETURNING reserved_micros")
                .bind(tenant).bind(event).fetch_optional(&mut *tx).await?;
            let Some((amount,)) = amount else {
                return Err(LedgerError::State);
            };
            sqlx::query("UPDATE ohc_usage_accounts SET reserved_micros=reserved_micros-$2 WHERE tenant_id=$1 AND reserved_micros >= $2")
                .bind(tenant).bind(amount).execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(())
        })
    }

    pub async fn settle(
        &self,
        tenant: &str,
        event: &str,
        receipt: &UsageReceipt,
    ) -> Result<(), LedgerError> {
        if !identifier(tenant) || !identifier(event) || !identifier(&receipt.provider_request_id) {
            return Err(LedgerError::Invalid);
        }
        if let Some(counts) = &receipt.counts
            && (counts.input < 0
                || counts.output < 0
                || counts.cached_input < 0
                || counts.cached_input > counts.input)
        {
            return Err(LedgerError::Invalid);
        }
        let receipt_json = serde_json::to_string(receipt).map_err(|_| LedgerError::Invalid)?;
        let receipt_digest = digest(receipt)?;
        transaction!(self, tenant, tx, {
            let acquired: Option<(String,i64)> = sqlx::query_as("UPDATE ohc_usage_records SET state='settling' WHERE tenant_id=$1 AND event_id=$2 AND state IN ('in_flight','reconciliation_required') RETURNING scope_json,reserved_micros")
                .bind(tenant).bind(event).fetch_optional(&mut *tx).await?;
            let Some((scope_json, reserved)) = acquired else {
                let previous: Option<(String,Option<String>)> = sqlx::query_as("SELECT state,receipt_digest FROM ohc_usage_records WHERE tenant_id=$1 AND event_id=$2")
                    .bind(tenant).bind(event).fetch_optional(&mut *tx).await?;
                return match previous {
                    Some((state, Some(hash))) if state == "settled" && hash == receipt_digest => {
                        Ok(())
                    }
                    Some((state, _)) if state == "settled" => Err(LedgerError::Conflict),
                    _ => Err(LedgerError::State),
                };
            };
            let scope: UsageScope =
                serde_json::from_str(&scope_json).map_err(|_| LedgerError::Database)?;
            let provider_cost = match (&scope.rate_card, &receipt.counts) {
                (Some(rate), Some(counts)) => Some(rate.cost(counts)?),
                _ => None,
            };
            // Even BYOK reports missing provider quantities as unknown, not free.
            let charge = if receipt.counts.is_none()
                || receipt.provider_request_id.starts_with("unknown:")
            {
                None
            } else if scope.payer == PayerMode::ManagedApi {
                provider_cost
            } else {
                Some(0)
            };
            if charge.is_none() || charge.is_some_and(|value| value > reserved) {
                sqlx::query("UPDATE ohc_usage_records SET state='reconciliation_required',receipt_json=$3,receipt_digest=$4,provider_cost_micros=$5 WHERE tenant_id=$1 AND event_id=$2")
                    .bind(tenant).bind(event).bind(&receipt_json).bind(&receipt_digest).bind(provider_cost).execute(&mut *tx).await?;
                tx.commit().await?;
                return Err(LedgerError::State);
            }
            let receipt_claimed = sqlx::query("INSERT INTO ohc_usage_receipts(tenant_id,provider,provider_request_id,event_id) VALUES($1,$2,$3,$4) ON CONFLICT(tenant_id,provider,provider_request_id) DO NOTHING")
                .bind(tenant).bind(&scope.provider).bind(&receipt.provider_request_id).bind(event)
                .execute(&mut *tx).await?.rows_affected();
            if receipt_claimed != 1 {
                return Err(LedgerError::Conflict);
            }
            let charge = charge.ok_or(LedgerError::State)?;
            let changed = sqlx::query("UPDATE ohc_usage_accounts SET reserved_micros=reserved_micros-$2,spent_micros=spent_micros+$3 WHERE tenant_id=$1 AND reserved_micros >= $2 AND spent_micros <= limit_micros-$3")
                .bind(tenant).bind(reserved).bind(charge).execute(&mut *tx).await?.rows_affected();
            if changed != 1 {
                return Err(LedgerError::State);
            }
            sqlx::query("UPDATE ohc_usage_records SET state='settled',charged_micros=$3,provider_cost_micros=$4,receipt_json=$5,receipt_digest=$6 WHERE tenant_id=$1 AND event_id=$2")
                .bind(tenant).bind(event).bind(charge).bind(provider_cost).bind(&receipt_json).bind(&receipt_digest).execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(())
        })
    }

    pub async fn summary(&self, tenant: &str) -> Result<AccountSummary, LedgerError> {
        if !identifier(tenant) {
            return Err(LedgerError::Invalid);
        }
        transaction!(self, tenant, tx, {
            let values: Option<(i64,i64,i64)> = sqlx::query_as("SELECT limit_micros,spent_micros,reserved_micros FROM ohc_usage_accounts WHERE tenant_id=$1")
                .bind(tenant).fetch_optional(&mut *tx).await?;
            let (limit_micros, spent_micros, reserved_micros) =
                values.ok_or(LedgerError::Unconfigured)?;
            Ok(AccountSummary {
                limit_micros,
                spent_micros,
                reserved_micros,
            })
        })
    }

    /// Bounded diagnostic/export page, never includes prompts or provider secrets.
    pub async fn records(
        &self,
        tenant: &str,
        after: &str,
    ) -> Result<Vec<UsageRecord>, LedgerError> {
        if !identifier(tenant) || after.len() > 255 {
            return Err(LedgerError::Invalid);
        }
        transaction!(self, tenant, tx, {
            type StoredUsage = (
                String,
                String,
                String,
                i64,
                Option<i64>,
                Option<i64>,
                Option<String>,
            );
            let rows: Vec<StoredUsage> = sqlx::query_as("SELECT event_id,state,scope_json,reserved_micros,charged_micros,provider_cost_micros,receipt_json FROM ohc_usage_records WHERE tenant_id=$1 AND event_id>$2 ORDER BY event_id LIMIT 100")
                .bind(tenant).bind(after).fetch_all(&mut *tx).await?;
            rows.into_iter()
                .map(
                    |(
                        event_id,
                        state,
                        scope,
                        reserved_micros,
                        charged_micros,
                        provider_cost_micros,
                        receipt,
                    )| {
                        Ok(UsageRecord {
                            event_id,
                            state,
                            scope: serde_json::from_str(&scope)
                                .map_err(|_| LedgerError::Database)?,
                            reserved_micros,
                            charged_micros,
                            provider_cost_micros,
                            receipt: receipt
                                .map(|value| serde_json::from_str(&value))
                                .transpose()
                                .map_err(|_| LedgerError::Database)?,
                        })
                    },
                )
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn ledger() -> UsageLedger {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let ledger = UsageLedger::Sqlite(pool);
        ledger.initialize().await.unwrap();
        ledger.set_limit("tenant-a", 1000).await.unwrap();
        ledger
    }
    fn scope(payer: PayerMode) -> UsageScope {
        UsageScope {
            tenant_id: "tenant-a".into(),
            task_id: "task".into(),
            attempt_id: "attempt".into(),
            provider: "test".into(),
            model: "model".into(),
            payer,
            rate_card: Some(RateCard {
                revision: "test-rate-1".into(),
                input_micros_per_million: 1_000_000,
                output_micros_per_million: 2_000_000,
                cached_input_micros_per_million: 100_000,
            }),
        }
    }
    fn receipt() -> UsageReceipt {
        UsageReceipt {
            provider_request_id: "response-1".into(),
            counts: Some(TokenCounts {
                input: 100,
                output: 50,
                cached_input: 20,
            }),
        }
    }
    fn request() -> String {
        "a".repeat(64)
    }
    #[tokio::test]
    async fn idempotent_settlement_and_tenant_isolation() {
        let ledger = ledger().await;
        let scope = scope(PayerMode::ManagedApi);
        assert_eq!(
            ledger
                .reserve(&scope, "event-1", &request(), 500)
                .await
                .unwrap(),
            Admission::Admitted
        );
        assert_eq!(
            ledger
                .reserve(&scope, "event-1", &request(), 500)
                .await
                .unwrap(),
            Admission::Duplicate
        );
        assert_eq!(
            ledger
                .reserve(&scope, "event-1", &"b".repeat(64), 500)
                .await
                .unwrap_err(),
            LedgerError::Conflict
        );
        ledger.dispatched("tenant-a", "event-1").await.unwrap();
        assert!(ledger.settle("other", "event-1", &receipt()).await.is_err());
        ledger
            .settle("tenant-a", "event-1", &receipt())
            .await
            .unwrap();
        ledger
            .settle("tenant-a", "event-1", &receipt())
            .await
            .unwrap();
        assert_eq!(
            ledger.summary("tenant-a").await.unwrap(),
            AccountSummary {
                limit_micros: 1000,
                spent_micros: 182,
                reserved_micros: 0
            }
        );
        assert!(ledger.records("other", "").await.unwrap().is_empty());
    }
    #[tokio::test]
    async fn byok_is_visible_but_not_rebilled() {
        let ledger = ledger().await;
        let scope = scope(PayerMode::ByokApi);
        assert!(
            ledger
                .reserve(&scope, "event", &request(), 1)
                .await
                .is_err()
        );
        ledger
            .reserve(&scope, "event", &request(), 0)
            .await
            .unwrap();
        ledger.dispatched("tenant-a", "event").await.unwrap();
        ledger
            .settle("tenant-a", "event", &receipt())
            .await
            .unwrap();
        let record = ledger.records("tenant-a", "").await.unwrap().remove(0);
        assert_eq!(record.charged_micros, Some(0));
        assert_eq!(record.provider_cost_micros, Some(182));
    }
    #[tokio::test]
    async fn concurrent_reservations_and_cancellation_obey_exposure() {
        let ledger = ledger().await;
        let mut workers = Vec::new();
        for i in 0..20 {
            let ledger = ledger.clone();
            workers.push(tokio::spawn(async move {
                ledger
                    .reserve(
                        &scope(PayerMode::ManagedApi),
                        &format!("e-{i:02}"),
                        &request(),
                        100,
                    )
                    .await
            }));
        }
        let mut accepted = 0;
        for worker in workers {
            if worker.await.unwrap().is_ok() {
                accepted += 1;
            }
        }
        assert_eq!(accepted, 10);
        assert_eq!(
            ledger.summary("tenant-a").await.unwrap().reserved_micros,
            1000
        );
        let records = ledger.records("tenant-a", "").await.unwrap();
        assert_eq!(records.len(), 10);
        ledger
            .cancel_before_dispatch("tenant-a", &records[0].event_id)
            .await
            .unwrap();
        assert_eq!(
            ledger.summary("tenant-a").await.unwrap().reserved_micros,
            900
        );
        assert!(
            ledger
                .cancel_before_dispatch("tenant-a", &records[0].event_id)
                .await
                .is_err()
        );
    }
    #[tokio::test]
    async fn unknown_outcome_keeps_reservation_until_reconciliation() {
        let ledger = ledger().await;
        ledger
            .reserve(&scope(PayerMode::ManagedApi), "event", &request(), 500)
            .await
            .unwrap();
        ledger.dispatched("tenant-a", "event").await.unwrap();
        assert!(
            ledger
                .cancel_before_dispatch("tenant-a", "event")
                .await
                .is_err()
        );
        assert_eq!(
            ledger
                .settle(
                    "tenant-a",
                    "event",
                    &UsageReceipt {
                        provider_request_id: "request-1".into(),
                        counts: None
                    }
                )
                .await
                .unwrap_err(),
            LedgerError::State
        );
        assert_eq!(
            ledger.summary("tenant-a").await.unwrap().reserved_micros,
            500
        );
        assert_eq!(
            ledger.records("tenant-a", "").await.unwrap()[0].state,
            "reconciliation_required"
        );
        ledger
            .settle("tenant-a", "event", &receipt())
            .await
            .unwrap();
        assert_eq!(ledger.summary("tenant-a").await.unwrap().spent_micros, 182);
    }
    #[tokio::test]
    async fn provider_receipt_cannot_be_charged_under_two_event_ids() {
        let ledger = ledger().await;
        for event in ["first", "second"] {
            ledger
                .reserve(&scope(PayerMode::ManagedApi), event, &request(), 300)
                .await
                .unwrap();
            ledger.dispatched("tenant-a", event).await.unwrap();
        }
        ledger
            .settle("tenant-a", "first", &receipt())
            .await
            .unwrap();
        assert_eq!(
            ledger
                .settle("tenant-a", "second", &receipt())
                .await
                .unwrap_err(),
            LedgerError::Conflict
        );
        assert_eq!(ledger.summary("tenant-a").await.unwrap().spent_micros, 182);
        assert_eq!(
            ledger.summary("tenant-a").await.unwrap().reserved_micros,
            300
        );
    }

    #[tokio::test]
    async fn reservations_and_replay_survive_connection_restart() {
        let path = std::env::temp_dir().join(format!("ohc-usage-{}.sqlite", uuid::Uuid::new_v4()));
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options.clone())
            .await
            .unwrap();
        let first = UsageLedger::Sqlite(pool.clone());
        first.initialize().await.unwrap();
        first.set_limit("tenant-a", 1000).await.unwrap();
        first
            .reserve(&scope(PayerMode::ManagedApi), "restart", &request(), 300)
            .await
            .unwrap();
        first.dispatched("tenant-a", "restart").await.unwrap();
        pool.close().await;
        let reopened_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .unwrap();
        let reopened = UsageLedger::Sqlite(reopened_pool.clone());
        assert_eq!(
            reopened.summary("tenant-a").await.unwrap().reserved_micros,
            300
        );
        assert_eq!(
            reopened
                .reserve(&scope(PayerMode::ManagedApi), "restart", &request(), 300)
                .await
                .unwrap(),
            Admission::Duplicate
        );
        reopened
            .settle("tenant-a", "restart", &receipt())
            .await
            .unwrap();
        assert_eq!(
            reopened.summary("tenant-a").await.unwrap().spent_micros,
            182
        );
        reopened_pool.close().await;
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn checked_money_counts_and_subcent_precision() {
        let rate = scope(PayerMode::ManagedApi).rate_card.unwrap();
        assert_eq!(
            rate.cost(&TokenCounts {
                input: 1,
                output: 0,
                cached_input: 0
            })
            .unwrap(),
            1
        );
        assert!(
            rate.cost(&TokenCounts {
                input: 1,
                output: 0,
                cached_input: 2
            })
            .is_err()
        );
        assert!(
            rate.cost(&TokenCounts {
                input: -1,
                output: 0,
                cached_input: 0
            })
            .is_err()
        );
        assert!(
            rate.cost(&TokenCounts {
                input: i64::MAX,
                output: i64::MAX,
                cached_input: 0
            })
            .is_err()
        );
    }
}
