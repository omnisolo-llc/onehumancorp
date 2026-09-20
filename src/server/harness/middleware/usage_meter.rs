//! Request-scoped provider usage. Operator settings, never request payloads,
//! select the payer and tariff. Missing quantities remain reconciliation work.
use super::usage_ledger::{
    Admission, LedgerError, PayerMode, RateCard, TokenCounts, UsageLedger, UsageReceipt, UsageScope,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, sync::Arc, time::Duration};

#[derive(Clone, PartialEq)]
pub struct UsageMeterSettings {
    pub database_url: String,
    pub scope: UsageScope,
    pub max_request_micros: i64,
}
impl std::fmt::Debug for UsageMeterSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsageMeterSettings")
            .field("scope", &self.scope)
            .field("database_url", &"<configured>")
            .finish()
    }
}
impl UsageMeterSettings {
    pub fn from_environment(
        tenant: &str,
        task: &str,
        attempt: &str,
        provider: &str,
        model: &str,
    ) -> Result<Option<Self>, String> {
        let mode = match std::env::var("OMNISOLO_USAGE_PAYER") {
            Err(std::env::VarError::NotPresent) => return Ok(None),
            Ok(mode) => mode,
            Err(_) => return Err("Invalid usage payer configuration".into()),
        };
        let payer=match mode.as_str() {
            "managed_api"=>PayerMode::ManagedApi, "byok_api"=>PayerMode::ByokApi,
            _=>return Err("API proxy accepts managed_api or byok_api only; subscription sessions cannot be relayed".into()),
        };
        let database_url = std::env::var("OMNISOLO_USAGE_DATABASE_URL")
            .map_err(|_| "Usage ledger database is required")?;
        if !(database_url.starts_with("postgres://")
            || database_url.starts_with("postgresql://")
            || database_url.starts_with("sqlite:"))
        {
            return Err("Usage ledger requires PostgreSQL or SQLite".into());
        }
        let cards: BTreeMap<String, RateCard> = match std::env::var("OMNISOLO_USAGE_RATE_CARDS") {
            Ok(value) => {
                serde_json::from_str(&value).map_err(|_| "Invalid usage rate-card configuration")?
            }
            Err(std::env::VarError::NotPresent) => BTreeMap::new(),
            Err(_) => return Err("Invalid usage rate-card configuration".into()),
        };
        let rate_card = cards.get(&format!("{provider}/{model}")).cloned();
        if payer == PayerMode::ManagedApi && rate_card.is_none() {
            return Err("No approved rate card for this provider/model".into());
        }
        let max_request_micros = if payer == PayerMode::ManagedApi {
            let value = std::env::var("OMNISOLO_USAGE_MAX_REQUEST_MICROS")
                .map_err(|_| "A request spending ceiling is required")?
                .parse::<i64>()
                .map_err(|_| "Invalid request spending ceiling")?;
            if value <= 0 {
                return Err("Request spending ceiling must be positive".into());
            }
            value
        } else {
            0
        };
        Ok(Some(Self {
            database_url,
            max_request_micros,
            scope: UsageScope {
                tenant_id: tenant.into(),
                task_id: task.into(),
                attempt_id: attempt.into(),
                provider: provider.into(),
                model: model.into(),
                payer,
                rate_card,
            },
        }))
    }
    pub async fn connect(&self) -> Result<Arc<RequestMeter>, LedgerError> {
        let ledger = if self.database_url.starts_with("sqlite:") {
            use std::str::FromStr;
            let options = sqlx::sqlite::SqliteConnectOptions::from_str(&self.database_url)?
                .create_if_missing(true)
                .busy_timeout(Duration::from_secs(10))
                .foreign_keys(true);
            UsageLedger::Sqlite(
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(1)
                    .connect_with(options)
                    .await?,
            )
        } else {
            UsageLedger::Postgres(
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(2)
                    .connect(&self.database_url)
                    .await?,
            )
        };
        // No automatic budget, balance top-up or schema mutation at inference time.
        ledger.summary(&self.scope.tenant_id).await?;
        Ok(Arc::new(RequestMeter {
            ledger,
            scope: self.scope.clone(),
            maximum: self.max_request_micros,
        }))
    }
}

pub struct RequestMeter {
    pub ledger: UsageLedger,
    pub scope: UsageScope,
    pub maximum: i64,
}
impl RequestMeter {
    pub async fn admit(&self, request: &str, payload: &[u8]) -> Result<(), LedgerError> {
        let hash = format!("{:x}", Sha256::digest(payload));
        let reserve = if self.scope.payer == PayerMode::ManagedApi {
            let value: Value = serde_json::from_slice(payload).map_err(|_| LedgerError::Invalid)?;
            // Text inference only: media and hosted tools require verified rates.
            fn unsupported(value: &Value) -> bool {
                match value {
                    Value::Object(map) => map.iter().any(|(key, value)| {
                        (key == "type"
                            && value.as_str().is_some_and(|kind| {
                                kind.contains("image")
                                    || kind.contains("audio")
                                    || kind.contains("video")
                                    || kind.contains("file")
                                    || kind.contains("web_search")
                                    || kind.contains("computer")
                            }))
                            || unsupported(value)
                    }),
                    Value::Array(items) => items.iter().any(unsupported),
                    _ => false,
                }
            }
            // Server-side conversation history and multi-output requests cannot
            // be bounded from this payload alone. Unknown hosted tool tariffs
            // likewise require their own reservation implementation.
            if value
                .get("previous_response_id")
                .is_some_and(|v| !v.is_null())
                || value.get("conversation").is_some_and(|v| !v.is_null())
                || value.get("n").is_some_and(|v| v.as_i64() != Some(1))
                || value
                    .get("tools")
                    .and_then(Value::as_array)
                    .is_some_and(|tools| {
                        tools.iter().any(|tool| {
                            tool.get("type").and_then(Value::as_str) != Some("function")
                        })
                    })
            {
                return Err(LedgerError::Unconfigured);
            }
            if unsupported(&value) {
                return Err(LedgerError::Unconfigured);
            }
            let output = value
                .get("max_output_tokens")
                .or_else(|| value.get("max_completion_tokens"))
                .or_else(|| value.get("max_tokens"))
                .and_then(Value::as_i64)
                .filter(|count| *count > 0 && *count <= 1_000_000)
                .ok_or(LedgerError::Unconfigured)?;
            let estimated = self
                .scope
                .rate_card
                .as_ref()
                .ok_or(LedgerError::Unconfigured)?
                .cost(&TokenCounts {
                    input: i64::try_from(payload.len()).map_err(|_| LedgerError::Invalid)?,
                    output,
                    cached_input: 0,
                })?;
            if estimated > self.maximum {
                return Err(LedgerError::Limit);
            }
            estimated
        } else {
            0
        };
        match self
            .ledger
            .reserve(&self.scope, request, &hash, reserve)
            .await?
        {
            Admission::Admitted => Ok(()),
            Admission::Duplicate => Err(LedgerError::Conflict),
        }
    }
    pub async fn dispatched(&self, request: &str) -> Result<(), LedgerError> {
        self.ledger.dispatched(&self.scope.tenant_id, request).await
    }
    pub async fn finish(&self, request: &str, capture: UsageCapture) -> Result<(), LedgerError> {
        self.ledger
            .settle(&self.scope.tenant_id, request, &capture.receipt(request))
            .await
    }
}

/// Retained output is bounded and discarded after quantities are extracted.
/// Neither prompts nor generated content are stored in the usage ledger.
#[derive(Default)]
pub struct UsageCapture {
    bytes: Vec<u8>,
    overflow: bool,
    pub provider_request_id: Option<String>,
}
impl UsageCapture {
    pub fn invalidate(&mut self) {
        self.bytes.clear();
        self.overflow = true;
    }
    pub fn push(&mut self, bytes: &[u8]) {
        if self.overflow {
            return;
        }
        if self.bytes.len().saturating_add(bytes.len()) > 8 * 1024 * 1024 {
            self.bytes.clear();
            self.overflow = true;
            return;
        }
        self.bytes.extend_from_slice(bytes);
    }
    pub fn receipt(self, fallback: &str) -> UsageReceipt {
        let mut id = self.provider_request_id;
        let mut counts = None;
        if !self.overflow {
            if let Ok(value) = serde_json::from_slice::<Value>(&self.bytes) {
                extract(&value, &mut id, &mut counts);
            } else if let Ok(text) = std::str::from_utf8(&self.bytes) {
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data:").map(str::trim)
                        && let Ok(value) = serde_json::from_str::<Value>(data)
                    {
                        extract(&value, &mut id, &mut counts);
                    }
                }
            }
        }
        UsageReceipt {
            provider_request_id: id
                .filter(|v| !v.is_empty() && v.len() <= 255)
                .unwrap_or_else(|| format!("unknown:{fallback}")),
            counts,
        }
    }
}
fn extract(value: &Value, id: &mut Option<String>, counts: &mut Option<TokenCounts>) {
    let value = value.get("response").unwrap_or(value);
    if let Some(provider_id) = value.get("id").and_then(Value::as_str) {
        *id = Some(provider_id.to_owned());
    }
    let Some(usage) = value.get("usage") else {
        return;
    };
    // A later malformed usage object invalidates any earlier partial snapshot.
    // Missing/null intermediate streaming usage is not a final observation.
    if usage.is_null() {
        return;
    }
    *counts = None;
    if !usage.is_object() {
        return;
    }
    let input = usage
        .get("input_tokens")
        .or_else(|| usage.get("prompt_tokens"))
        .and_then(Value::as_i64);
    let output = usage
        .get("output_tokens")
        .or_else(|| usage.get("completion_tokens"))
        .and_then(Value::as_i64);
    let cached = usage
        .pointer("/input_tokens_details/cached_tokens")
        .or_else(|| usage.pointer("/prompt_tokens_details/cached_tokens"));
    let cached = match cached {
        None => Some(0),
        Some(v) => v.as_i64(),
    };
    if let (Some(input), Some(output), Some(cached_input)) = (input, output, cached)
        && input >= 0
        && output >= 0
        && cached_input >= 0
        && cached_input <= input
    {
        *counts = Some(TokenCounts {
            input,
            output,
            cached_input,
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn usage_is_not_guessed_from_output_length() {
        let mut capture = UsageCapture::default();
        capture.push(br#"{"id":"r1","output_text":"hello"}"#);
        assert!(capture.receipt("local").counts.is_none());
    }
    #[test]
    fn split_stream_and_cached_input_are_preserved() {
        let input=b"data: {\"response\":{\"id\":\"r1\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3,\"input_tokens_details\":{\"cached_tokens\":5}}}}\n\ndata: [DONE]\n";
        let mut capture = UsageCapture::default();
        for chunk in input.chunks(7) {
            capture.push(chunk);
        }
        let receipt = capture.receipt("local");
        assert_eq!(receipt.provider_request_id, "r1");
        assert_eq!(
            receipt.counts,
            Some(TokenCounts {
                input: 12,
                output: 3,
                cached_input: 5
            })
        );
    }
    #[test]
    fn invalid_final_usage_cannot_reuse_an_earlier_stream_snapshot() {
        let mut capture = UsageCapture::default();
        capture.push(
            b"data: {\"id\":\"r1\",\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":2}}\n\n",
        );
        capture.push(
            b"data: {\"id\":\"r1\",\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":-1}}\n\n",
        );
        assert!(capture.receipt("event").counts.is_none());
    }
    #[tokio::test]
    async fn unbounded_provider_context_is_rejected_before_reservation() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let ledger = UsageLedger::Sqlite(pool);
        ledger.initialize().await.unwrap();
        ledger.set_limit("tenant", 1_000_000).await.unwrap();
        let meter = RequestMeter {
            ledger: ledger.clone(),
            maximum: 100_000,
            scope: UsageScope {
                tenant_id: "tenant".into(),
                task_id: "task".into(),
                attempt_id: "attempt".into(),
                provider: "openai".into(),
                model: "test".into(),
                payer: PayerMode::ManagedApi,
                rate_card: Some(RateCard {
                    revision: "test".into(),
                    input_micros_per_million: 1_000_000,
                    output_micros_per_million: 1_000_000,
                    cached_input_micros_per_million: 0,
                }),
            },
        };
        for extra in [
            serde_json::json!({"previous_response_id":"previous"}),
            serde_json::json!({"conversation":"stored"}),
            serde_json::json!({"n":2}),
            serde_json::json!({"tools":[{"type":"code_interpreter"}]}),
            serde_json::json!({"n":"1"}),
        ] {
            let mut value =
                serde_json::json!({"model":"test","input":"hello","max_output_tokens":100});
            value
                .as_object_mut()
                .unwrap()
                .extend(extra.as_object().unwrap().clone());
            assert_eq!(
                meter
                    .admit(
                        &uuid::Uuid::new_v4().to_string(),
                        &serde_json::to_vec(&value).unwrap()
                    )
                    .await
                    .unwrap_err(),
                LedgerError::Unconfigured
            );
        }
        assert_eq!(ledger.summary("tenant").await.unwrap().reserved_micros, 0);
        assert!(ledger.records("tenant", "").await.unwrap().is_empty());
    }
    #[test]
    fn malformed_negative_or_oversized_usage_stays_unknown() {
        let mut capture = UsageCapture::default();
        capture.push(br#"{"usage":{"prompt_tokens":1,"completion_tokens":-1}}"#);
        assert!(capture.receipt("local").counts.is_none());
        let mut capture = UsageCapture::default();
        capture.push(&vec![0; 8 * 1024 * 1024 + 1]);
        assert!(capture.receipt("local").counts.is_none());
    }
}
