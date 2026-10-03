//! Operator-authored credential ownership, payer and tariff. User payloads never
//! select these values or create a spending allowance. This ledger uses exactly
//! the same configured SQLx pool as the receipt store, with separate, conservative
//! transactions: an ambiguous admission is retained and is never redispatched.
use super::receipts::{Error, Receipt};
use super::{AdmissionError, AdmittedAnalysis, AnalysisPolicy, Authority};
use crate::persistence::AppDatabase;
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use server_harness::middleware::usage_ledger::{
    Admission, LedgerError, PayerMode, RateCard, TokenCounts, UsageLedger, UsageReceipt, UsageScope,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FundingPolicy {
    pub operator_tenant: String,
    pub payer: PayerMode,
    pub rate_card: Option<RateCard>,
    pub request_ceiling_micros: i64,
}
impl FundingPolicy {
    pub(crate) fn from_environment(policy: &AnalysisPolicy) -> Result<Self, AdmissionError> {
        Self::from_values(policy, |key| std::env::var(key).ok())
    }
    fn from_values(
        policy: &AnalysisPolicy,
        lookup: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, AdmissionError> {
        let tenant = lookup("OMNISOLO_LLM_TENANT_ID")
            .or_else(|| lookup("OMNISOLO_BUILDER_TENANT_ID"))
            .ok_or(AdmissionError::Unavailable)?;
        let payer = match lookup("OMNISOLO_USAGE_PAYER").as_deref() {
            Some("managed_api") => PayerMode::ManagedApi,
            Some("byok_api") => PayerMode::ByokApi,
            Some("local") => PayerMode::Local,
            _ => return Err(AdmissionError::Unavailable),
        };
        let cards: BTreeMap<String, RateCard> = lookup("OMNISOLO_USAGE_RATE_CARDS")
            .map(|v| serde_json::from_str(&v))
            .transpose()
            .map_err(|_| AdmissionError::Unavailable)?
            .unwrap_or_default();
        let rate_card = cards
            .get(&format!("{}/{}", policy.provider, policy.model))
            .cloned();
        let request_ceiling_micros = if payer == PayerMode::ManagedApi {
            lookup("OMNISOLO_USAGE_MAX_REQUEST_MICROS")
                .ok_or(AdmissionError::Unavailable)?
                .parse()
                .map_err(|_| AdmissionError::Unavailable)?
        } else {
            0
        };
        let value = Self {
            operator_tenant: tenant,
            payer,
            rate_card,
            request_ceiling_micros,
        };
        value
            .validate(policy)
            .map_err(|_| AdmissionError::Unavailable)?;
        Ok(value)
    }
    pub(crate) fn validate(&self, policy: &AnalysisPolicy) -> Result<(), Error> {
        if self.operator_tenant.trim().is_empty()
            || self.operator_tenant.trim() != self.operator_tenant
            || self.operator_tenant.len() > 255
            || self.operator_tenant.eq_ignore_ascii_case("system")
            || self.operator_tenant.chars().any(char::is_control)
            || self.payer == PayerMode::NativeSubscription
            || (self.payer == PayerMode::Local) != (policy.provider == "ollama")
            || (self.payer == PayerMode::ManagedApi
                && (self.rate_card.is_none()
                    || !(1..=1_000_000_000_000).contains(&self.request_ceiling_micros)))
            || (self.payer != PayerMode::ManagedApi && self.request_ceiling_micros != 0)
        {
            return Err(Error::Invalid);
        }
        if let Some(rate) = &self.rate_card {
            rate.cost(&TokenCounts {
                input: 0,
                output: 0,
                cached_input: 0,
            })
            .map_err(|_| Error::Invalid)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct UsageTicket {
    pub event_id: String,
    scope: UsageScope,
    digest: String,
    maximum: i64,
}
#[derive(Clone)]
pub(crate) struct FundingContext {
    pub policy: FundingPolicy,
    pub ledger: UsageLedger,
}
impl FundingContext {
    pub(crate) fn new(
        policy: FundingPolicy,
        database: &AppDatabase,
    ) -> Result<Self, AdmissionError> {
        let ledger = match database.connection().get_database_backend() {
            sea_orm::DatabaseBackend::Postgres => {
                UsageLedger::Postgres(database.connection().get_postgres_connection_pool().clone())
            }
            sea_orm::DatabaseBackend::Sqlite => {
                UsageLedger::Sqlite(database.connection().get_sqlite_connection_pool().clone())
            }
            _ => return Err(AdmissionError::Unavailable),
        };
        Ok(Self { policy, ledger })
    }
    fn ticket(
        &self,
        identity: (&str, &str),
        task: &str,
        policy: &AnalysisPolicy,
        event: String,
        request: &str,
        prepared: (i64, &str),
    ) -> Result<UsageTicket, Error> {
        let (tenant, actor) = identity;
        let (input, prepared_digest) = prepared;
        self.policy.validate(policy)?;
        if tenant != self.policy.operator_tenant {
            return Err(Error::Forbidden);
        }
        // The adapter derives this bound from its exact normalized request,
        // including all system/user text plus fixed protocol framing. Reserve
        // every allowed output token and the higher permitted input tariff.
        // Cached input is a subset, but the rate card need not discount it.
        // Hidden request sources are rejected before a receipt is admitted.
        let maximum = if self.policy.payer == PayerMode::ManagedApi {
            if !(4096..=1_000_000).contains(&input) {
                return Err(Error::Invalid);
            }
            let rate = self.policy.rate_card.as_ref().ok_or(Error::Unavailable)?;
            let cost = |cached_input| {
                rate.cost(&TokenCounts {
                    input,
                    output: i64::from(policy.max_output_tokens),
                    cached_input,
                })
                .map_err(|_| Error::Invalid)
            };
            let amount = cost(0)?.max(cost(input)?);
            if amount > self.policy.request_ceiling_micros {
                return Err(Error::Budget);
            }
            amount
        } else {
            0
        };
        let scope = UsageScope {
            tenant_id: tenant.into(),
            task_id: event.clone(),
            attempt_id: "1".into(),
            provider: policy.provider.clone(),
            model: policy.model.clone(),
            payer: self.policy.payer,
            rate_card: self.policy.rate_card.clone(),
        };
        let digest = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(
                    "ohc-workflow-usage-v1",
                    tenant,
                    actor,
                    request,
                    task,
                    &scope,
                    &self.policy,
                    input,
                    prepared_digest
                ))
                .map_err(|_| Error::Invalid)?
            )
        );
        Ok(UsageTicket {
            event_id: event,
            scope,
            digest,
            maximum,
        })
    }
    async fn reserve_ticket(&self, ticket: UsageTicket) -> Result<UsageTicket, Error> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.ledger.reserve(
                &ticket.scope,
                &ticket.event_id,
                &ticket.digest,
                ticket.maximum,
            ),
        )
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(ledger_error)?
        {
            Admission::Admitted => Ok(ticket),
            // Only a receipt replay may reuse an identity; it never dispatches.
            Admission::Duplicate => Err(Error::Conflict),
        }
    }
    pub(crate) async fn reserve_receipt(&self, receipt: &Receipt) -> Result<UsageTicket, Error> {
        if receipt.funding.as_ref() != Some(&self.policy) {
            return Err(Error::Conflict);
        }
        let policy = AnalysisPolicy::new(
            receipt.provider.clone(),
            receipt.model.clone(),
            receipt.max_output_tokens,
        )?;
        let ticket = self.ticket(
            (&receipt.tenant_id, &receipt.actor_id),
            &receipt.task,
            &policy,
            receipt.id.clone(),
            &receipt.request_id,
            (
                receipt.input_token_bound.ok_or(Error::Unavailable)?,
                receipt
                    .prepared_request_digest
                    .as_deref()
                    .ok_or(Error::Unavailable)?,
            ),
        )?;
        self.reserve_ticket(ticket).await
    }
    pub(crate) async fn reserve_direct(
        &self,
        input: &AdmittedAnalysis,
    ) -> Result<UsageTicket, Error> {
        let Authority {
            tenant_id,
            actor_id,
            ..
        } = &input.authority;
        self.reserve_ticket(self.ticket(
            (tenant_id, actor_id),
            &input.task,
            &input.policy,
            input.execution_id.clone(),
            &input.execution_id,
            (
                input.input_token_bound.ok_or(Error::Unavailable)?,
                input.prepared.as_ref().ok_or(Error::Unavailable)?.digest(),
            ),
        )?)
        .await
    }
    pub(crate) async fn cancel_reserved(&self, ticket: &UsageTicket) {
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.ledger
                .cancel_before_dispatch(&ticket.scope.tenant_id, &ticket.event_id),
        )
        .await;
        observe_write(&ticket.event_id, "cancel_before_dispatch", result);
    }
    pub(crate) async fn dispatched(&self, ticket: &UsageTicket) -> Result<(), Error> {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.ledger
                .dispatched(&ticket.scope.tenant_id, &ticket.event_id),
        )
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(ledger_error)
    }
    pub(crate) async fn settle(&self, ticket: &UsageTicket, receipt: &UsageReceipt) {
        // Unknown quantities, over-reservation responses or ambiguous commits
        // retain exposure in the ledger. No estimate is substituted as a bill.
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.ledger
                .settle(&ticket.scope.tenant_id, &ticket.event_id, receipt),
        )
        .await;
        observe_write(&ticket.event_id, "settle", result);
    }
    pub(crate) async fn unknown(&self, ticket: &UsageTicket) {
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.ledger
                .require_reconciliation(&ticket.scope.tenant_id, &ticket.event_id),
        )
        .await;
        observe_write(&ticket.event_id, "require_reconciliation", result);
    }
}
/// Log only stable internal IDs and redacted error categories. A failed or
/// ambiguous write leaves the persisted hold available for reconciliation.
pub(super) fn observe_write<T>(
    event_id: &str,
    operation: &'static str,
    result: Result<Result<T, LedgerError>, tokio::time::error::Elapsed>,
) {
    match result {
        Ok(Ok(_)) => {}
        Ok(Err(LedgerError::State)) => {
            tracing::debug!(event_id, operation, "usage state requires reconciliation")
        }
        Ok(Err(error)) => {
            tracing::warn!(event_id, operation, error=%error, "usage write unconfirmed; retained exposure requires reconciliation")
        }
        Err(_) => tracing::warn!(
            event_id,
            operation,
            "usage write timed out; retained exposure requires reconciliation"
        ),
    }
}
fn ledger_error(error: LedgerError) -> Error {
    match error {
        LedgerError::Limit => Error::Budget,
        LedgerError::Invalid => Error::Invalid,
        LedgerError::Conflict | LedgerError::State => Error::Conflict,
        _ => Error::Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn admission_reserves_the_worst_permitted_cached_or_uncached_input_tariff() {
        use sea_orm::SqlxSqliteConnector;
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        let database =
            AppDatabase::from_connection(SqlxSqliteConnector::from_sqlx_sqlite_pool(pool));
        let policy =
            AnalysisPolicy::new("openai-compatible".into(), "explicit-model".into(), 256).unwrap();
        for (ordinary, cached) in [(1_000_000, 2_000_000), (2_000_000, 1_000_000)] {
            let mut funding = FundingContext::new(
                FundingPolicy {
                    operator_tenant: "operator".into(),
                    payer: PayerMode::ManagedApi,
                    rate_card: Some(RateCard {
                        revision: "explicit-test-tariff".into(),
                        input_micros_per_million: ordinary,
                        output_micros_per_million: 3_000_000,
                        cached_input_micros_per_million: cached,
                    }),
                    request_ceiling_micros: 20_000,
                },
                &database,
            )
            .unwrap();
            let ticket = funding
                .ticket(
                    ("operator", "owner"),
                    "Submitted text",
                    &policy,
                    "event".into(),
                    "request",
                    (4096, &"a".repeat(64)),
                )
                .unwrap();
            assert_eq!(
                ticket.maximum,
                4096 * 2 + 256 * 3,
                "cache is a subset of input, but no discounted tariff may be assumed"
            );
            funding.policy.request_ceiling_micros = 8000;
            assert!(matches!(
                funding.ticket(
                    ("operator", "owner"),
                    "Submitted text",
                    &policy,
                    "event".into(),
                    "request",
                    (4096, &"a".repeat(64))
                ),
                Err(Error::Budget)
            ));
            funding.policy.request_ceiling_micros = 20_000;
            funding
                .policy
                .rate_card
                .as_mut()
                .unwrap()
                .cached_input_micros_per_million = i64::MAX;
            assert!(
                funding
                    .ticket(
                        ("operator", "owner"),
                        "Submitted text",
                        &policy,
                        "event".into(),
                        "request",
                        (4096, &"a".repeat(64))
                    )
                    .is_err()
            );
        }
    }
    #[test]
    fn operator_configuration_cannot_infer_a_tenant_payer_tariff_or_subscription_permission() {
        let remote =
            AnalysisPolicy::new("openai-compatible".into(), "explicit-model".into(), 128).unwrap();
        let local = AnalysisPolicy::new("ollama".into(), "explicit-model".into(), 128).unwrap();
        let from = |policy: &AnalysisPolicy, pairs: &[(&str, &str)]| {
            FundingPolicy::from_values(policy, |key| {
                pairs
                    .iter()
                    .find(|(name, _)| *name == key)
                    .map(|(_, value)| value.to_string())
            })
        };
        assert!(from(&remote, &[]).is_err());
        assert!(from(&remote, &[("OMNISOLO_LLM_TENANT_ID", "operator")]).is_err());
        assert!(
            from(
                &remote,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "managed_api"),
                    ("OMNISOLO_USAGE_MAX_REQUEST_MICROS", "1000")
                ]
            )
            .is_err()
        );
        assert!(
            from(
                &remote,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "native_subscription")
                ]
            )
            .is_err()
        );
        assert!(
            from(
                &remote,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "local")
                ]
            )
            .is_err()
        );
        assert!(
            from(
                &local,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "byok_api")
                ]
            )
            .is_err()
        );
        assert!(
            from(
                &local,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "system"),
                    ("OMNISOLO_USAGE_PAYER", "local")
                ]
            )
            .is_err()
        );
        assert!(
            from(
                &local,
                &[
                    ("OMNISOLO_LLM_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "local")
                ]
            )
            .is_ok()
        );
        assert!(
            from(
                &remote,
                &[
                    ("OMNISOLO_BUILDER_TENANT_ID", "operator"),
                    ("OMNISOLO_USAGE_PAYER", "byok_api")
                ]
            )
            .is_ok()
        );
    }
}
