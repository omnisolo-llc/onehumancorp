//! Canonical reviewed inbox reply -> one durable provider attempt -> acceptance.
//! No detached sends, caller-selected recipients, delivery claims or automatic
//! retry after an uncertain outcome. Both department and feed workers use this.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub enum Store {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}
impl Store {
    pub fn from_db(db: &crate::db::DB) -> Self {
        match &db.store {
            crate::db::DbStore::Postgres => Self::Postgres(db.pool.clone()),
            crate::db::DbStore::Sqlite(pool) => Self::Sqlite(pool.clone()),
        }
    }
    async fn begin(&self, tenant: &str) -> Result<Transaction, Error> {
        if tenant.trim().is_empty() {
            return Err(Error::Invalid("Tenant identity required"));
        }
        Ok(match self {
            Self::Postgres(pool) => {
                let mut tx = pool.begin().await?;
                sqlx::query("SELECT set_config('app.current_tenant',$1,true)")
                    .bind(tenant)
                    .execute(&mut *tx)
                    .await?;
                Transaction::Postgres(tx)
            }
            Self::Sqlite(pool) => {
                let mut tx = pool.begin().await?;
                // Reserve the writer before reading authorization or the replay fence.
                sqlx::query("UPDATE department_message_dispatches SET state=state WHERE 0")
                    .execute(&mut *tx)
                    .await?;
                Transaction::Sqlite(tx)
            }
        })
    }
}
enum Transaction {
    Postgres(sqlx::Transaction<'static, sqlx::Postgres>),
    Sqlite(sqlx::Transaction<'static, sqlx::Sqlite>),
}
impl Transaction {
    async fn commit(self) -> Result<(), Error> {
        match self {
            Self::Postgres(tx) => tx.commit().await?,
            Self::Sqlite(tx) => tx.commit().await?,
        };
        Ok(())
    }
}
macro_rules! database {
    ($tx:expr, $connection:ident, $body:block) => {{
        match &mut $tx {
            Transaction::Postgres(tx) => {
                let $connection = &mut **tx;
                $body
            }
            Transaction::Sqlite(tx) => {
                let $connection = &mut **tx;
                $body
            }
        }
    }};
}
#[derive(Debug)]
pub enum Error {
    Storage,
    Invalid(&'static str),
}
impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Self::Storage
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage => {
                f.write_str("Message receipt storage unavailable; reconcile before retrying")
            }
            Self::Invalid(message) => f.write_str(message),
        }
    }
}
impl std::error::Error for Error {}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Binding {
    credential_id: String,
    integration_id: String,
    account: String,
    from: String,
    source: String,
}
#[derive(sqlx::FromRow)]
struct Credential {
    id: String,
    integration_id: String,
    bot_token: Option<String>,
    api_token: Option<String>,
    from_phone: Option<String>,
}
impl Credential {
    fn binding(&self, source: &str) -> Binding {
        Binding {
            credential_id: self.id.clone(),
            integration_id: self.integration_id.clone(),
            account: if self.integration_id == "meta" || self.integration_id == "whatsapp_cloud_api"
            {
                self.from_phone.clone().unwrap_or_default()
            } else {
                self.bot_token.clone().unwrap_or_default()
            },
            from: self.from_phone.clone().unwrap_or_default(),
            source: source.to_owned(),
        }
    }
    fn configured(&self, binding: &Binding) -> bool {
        if self
            .api_token
            .as_deref()
            .is_none_or(|value| value.trim().is_empty())
            || binding.from.trim().is_empty()
        {
            return false;
        }
        if matches!(self.integration_id.as_str(), "meta" | "whatsapp_cloud_api") {
            return binding.from.bytes().all(|b| b.is_ascii_digit());
        }
        binding.account.len() == 34
            && binding.account.starts_with("AC")
            && binding.account[2..].bytes().all(|b| b.is_ascii_hexdigit())
    }
}
#[derive(sqlx::FromRow)]
struct InboxRow {
    kind: String,
    source: Option<String>,
    sender_id: Option<String>,
    status: Option<String>,
}
struct Inbox {
    kind: String,
    source: Option<String>,
    sender_id: Option<String>,
    status: Option<String>,
    terminal: bool,
}
async fn inbox(mut tx: &mut Transaction, tenant: &str, id: &str) -> Result<Inbox, Error> {
    for table in ["inbox_messages", "omni_inbox_messages"] {
        let query = format!("UPDATE {table} SET status=status WHERE tenant_id=$1 AND id=$2");
        database!(tx, c, {
            sqlx::query(&query).bind(tenant).bind(id).execute(c).await?;
        });
    }
    let rows = database!(tx, c, {
        sqlx::query_as::<_,InboxRow>("SELECT 'inbox' AS kind,source,sender_id,status FROM inbox_messages WHERE tenant_id=$1 AND id=$2 UNION ALL SELECT 'omni' AS kind,source,sender_id,status FROM omni_inbox_messages WHERE tenant_id=$1 AND id=$2")
            .bind(tenant).bind(id).fetch_all(c).await?
    });
    let first = rows
        .first()
        .ok_or(Error::Invalid("Canonical inbox message not found"))?;
    if rows
        .iter()
        .any(|row| row.source != first.source || row.sender_id != first.sender_id)
    {
        return Err(Error::Invalid(
            "Inbox mirrors disagree; reconcile before dispatch",
        ));
    }
    let terminal = rows
        .iter()
        .any(|row| row.status.as_deref().is_some_and(terminal_state));
    let historical = rows
        .iter()
        .find_map(|row| {
            row.status
                .as_deref()
                .filter(|state| historical_state(state))
                .map(str::to_owned)
        })
        .or_else(|| {
            rows.iter().find_map(|row| {
                row.status
                    .as_deref()
                    .filter(|state| terminal_state(state))
                    .map(str::to_owned)
            })
        });
    let mut first = rows
        .into_iter()
        .next()
        .ok_or(Error::Invalid("Canonical inbox message not found"))?;
    if historical.is_some() {
        first.status = historical;
    }
    Ok(Inbox {
        kind: first.kind,
        source: first.source,
        sender_id: first.sender_id,
        status: first.status,
        terminal,
    })
}
async fn credential(
    mut tx: &mut Transaction,
    tenant: &str,
    source: &str,
    id: Option<&str>,
) -> Result<Option<Credential>, Error> {
    database!(tx, c, {
        sqlx::query("UPDATE integration_credentials SET id=id WHERE tenant_id=$1 AND (CAST($2 AS TEXT) IS NULL OR id=$2)").bind(tenant).bind(id).execute(c).await?;
    });
    Ok(database!(tx, c, {
        sqlx::query_as::<_,Credential>("SELECT id,integration_id,bot_token,api_token,from_phone FROM integration_credentials WHERE tenant_id=$1 AND (($2='whatsapp' AND integration_id IN ('whatsapp_cloud_api','whatsapp','twilio')) OR ($2 IN ('instagram','facebook') AND integration_id='meta') OR ($2='sms' AND integration_id='twilio')) AND (CAST($3 AS TEXT) IS NULL OR id=$3) ORDER BY CASE integration_id WHEN 'whatsapp_cloud_api' THEN 0 WHEN 'whatsapp' THEN 1 ELSE 2 END,id LIMIT 1")
            .bind(tenant).bind(source).bind(id).fetch_optional(c).await?
    }))
}
fn text<'a>(payload: &'a Value, key: &str) -> Result<&'a str, Error> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or(Error::Invalid(
            "Canonical reply identity or content is missing",
        ))
}
async fn manual_intent_revision(
    mut tx: &mut Transaction,
    tenant: &str,
    id: &str,
) -> Result<i64, Error> {
    let revision: Option<(i64,)> = database!(tx, c, {
        sqlx::query_as("SELECT revision FROM manual_inbox_intents WHERE tenant_id=$1 AND inbox_message_id=$2")
            .bind(tenant).bind(id).fetch_optional(c).await?
    });
    Ok(revision.map(|row| row.0).unwrap_or(0))
}
/// Freeze server-owned routing information before it is offered for review.
/// Only content is caller-authored; recipient and account are read from this tenant.
pub async fn prepare(store: &Store, tenant: &str, mut payload: Value) -> Result<Value, Error> {
    let id = text(&payload, "inbox_message_id")?;
    let mut tx = store.begin(tenant).await?;
    let row = inbox(&mut tx, tenant, id).await?;
    let revision = manual_intent_revision(&mut tx, tenant, id).await?;
    payload["manual_intent_revision"] = Value::from(revision);
    let source = row.source.unwrap_or_default();
    let target = row.sender_id.unwrap_or_default();
    let binding = credential(&mut tx, tenant, &source, None)
        .await?
        .map(|creds| creds.binding(&source));
    payload["source"] = Value::String(source);
    payload["sender_id"] = Value::String(target);
    payload["delivery_binding"] =
        serde_json::to_value(binding).map_err(|_| Error::Invalid("Invalid provider binding"))?;
    payload["delivery_authority"] = Value::String("owner_review_required".into());
    tx.commit().await?;
    Ok(payload)
}
/// A pending review marker for the existing triage producer. This does not
/// approve the action; dispatch still requires the canonical APPROVED feed row.
pub async fn record_pending_review(store: &Store, tenant: &str, action: &str) -> Result<(), Error> {
    let mut tx = store.begin(tenant).await?;
    database!(tx, c, {
        sqlx::query("INSERT INTO agent_action_requests(id,tenant_id,action_type,status,department_type,description) VALUES($1,$2,'HIGH','DRAFT','customer_success','Review canonical inbox reply')")
            .bind(action).bind(tenant).execute(c).await?;
    });
    tx.commit().await
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, sqlx::FromRow)]
pub struct DeliveryReceipt {
    pub state: String,
    pub provider_message_id: Option<String>,
    pub detail: String,
}
impl DeliveryReceipt {
    pub fn require_acceptance(self) -> Result<(), String> {
        if self.state == "accepted" && self.provider_message_id.is_some() {
            Ok(())
        } else {
            Err(format!("Message {}: {}", self.state, self.detail))
        }
    }
}
#[derive(Clone, Debug)]
enum SendFailure {
    Rejected,
    Unknown,
    Blocked,
}
#[async_trait]
trait DeliveryProvider: Send + Sync {
    async fn send(
        &self,
        binding: &Binding,
        recipient: &str,
        body: &str,
        credential: &Credential,
    ) -> Result<String, SendFailure>;
}
struct LiveProvider;
#[async_trait]
impl DeliveryProvider for LiveProvider {
    async fn send(
        &self,
        binding: &Binding,
        recipient: &str,
        body: &str,
        credential: &Credential,
    ) -> Result<String, SendFailure> {
        let token = credential.api_token.clone().ok_or(SendFailure::Blocked)?;
        if matches!(
            binding.integration_id.as_str(),
            "meta" | "whatsapp_cloud_api"
        ) {
            use crate::integrations::meta::{client::MetaSendError, provider::MetaProvider};
            let to = recipient.strip_prefix("whatsapp:").unwrap_or(recipient);
            MetaProvider::new(token, Some(binding.from.clone()))
                .send_message_receipt(&binding.source, to, body)
                .await
                .map(|receipt| receipt.message_id)
                .map_err(|error| match error {
                    MetaSendError::InvalidConfiguration => SendFailure::Blocked,
                    MetaSendError::Rejected { .. } => SendFailure::Rejected,
                    MetaSendError::UnknownOutcome { .. } => SendFailure::Unknown,
                })
        } else {
            use crate::integrations::twilio::{client::MessageSendError, provider::TwilioProvider};
            let provider = TwilioProvider::new(binding.account.clone(), token);
            let result = if binding.source == "whatsapp" {
                let to = format!(
                    "whatsapp:{}",
                    recipient.strip_prefix("whatsapp:").unwrap_or(recipient)
                );
                let from = format!(
                    "whatsapp:{}",
                    binding
                        .from
                        .strip_prefix("whatsapp:")
                        .unwrap_or(&binding.from)
                );
                provider.send_whatsapp(&to, &from, body).await
            } else {
                provider
                    .send_sms(
                        recipient.strip_prefix("sms:").unwrap_or(recipient),
                        &binding.from,
                        body,
                    )
                    .await
            };
            result
                .map(|receipt| receipt.sid)
                .map_err(|error| match error {
                    MessageSendError::Rejected { .. } | MessageSendError::OptedOut => {
                        SendFailure::Rejected
                    }
                    MessageSendError::UnknownOutcome { .. } => SendFailure::Unknown,
                })
        }
    }
}
fn terminal_state(state: &str) -> bool {
    matches!(
        state,
        "resolved" | "dismissed" | "closed" | "paused" | "cancelled" | "canceled"
    )
}
fn historical_state(state: &str) -> bool {
    matches!(
        state,
        "sent"
            | "replied"
            | "auto_replied"
            | "delivered"
            | "provider_accepted"
            | "delivery_unknown"
    )
}
fn valid_recipient(source: &str, recipient: &str) -> bool {
    let recipient = if source == "sms" {
        recipient.strip_prefix("sms:").unwrap_or(recipient)
    } else {
        recipient
    };
    let number = recipient
        .strip_prefix("whatsapp:")
        .unwrap_or(recipient)
        .trim_start_matches('+');
    match source {
        "sms" | "whatsapp" => {
            (5..=20).contains(&number.len()) && number.bytes().all(|b| b.is_ascii_digit())
        }
        "instagram" | "facebook" => {
            !recipient.is_empty()
                && recipient.len() <= 128
                && recipient.bytes().all(|b| b.is_ascii_digit())
        }
        _ => false,
    }
}
async fn set_inbox_state(
    mut tx: &mut Transaction,
    tenant: &str,
    id: &str,
    state: &str,
    body: &str,
) -> Result<(), Error> {
    for table in ["inbox_messages", "omni_inbox_messages"] {
        let query =
            format!("UPDATE {table} SET status=$1,draft_reply=$2 WHERE tenant_id=$3 AND id=$4");
        database!(tx, c, {
            sqlx::query(&query)
                .bind(state)
                .bind(body)
                .bind(tenant)
                .bind(id)
                .execute(c)
                .await?;
        });
    }
    Ok(())
}
pub async fn dispatch(store: &Store, tenant: &str, action: &str) -> Result<DeliveryReceipt, Error> {
    dispatch_with(store, tenant, action, &LiveProvider).await
}
pub async fn dispatch_admitted(
    store: &Store,
    tenant: &str,
    action: &str,
    job: &str,
    payload: &Value,
) -> Result<DeliveryReceipt, Error> {
    dispatch_inner(store, tenant, action, &LiveProvider, Some((job, payload))).await
}
async fn dispatch_with(
    store: &Store,
    tenant: &str,
    action: &str,
    provider: &dyn DeliveryProvider,
) -> Result<DeliveryReceipt, Error> {
    dispatch_inner(store, tenant, action, provider, None).await
}
async fn dispatch_inner(
    store: &Store,
    tenant: &str,
    action: &str,
    provider: &dyn DeliveryProvider,
    admitted: Option<(&str, &Value)>,
) -> Result<DeliveryReceipt, Error> {
    if action.trim().is_empty() {
        return Err(Error::Invalid("Canonical action identity required"));
    }
    let mut tx = store.begin(tenant).await?;
    // Follow the existing owner-decision lock order before examining its fence.
    if let Transaction::Postgres(pg) = &mut tx {
        sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-feed-decision-v1',$1::text,$2::text)::text,0))")
            .bind(tenant).bind(action).execute(&mut **pg).await?;
    }
    // The same row lock serializes revocation/edits with the provider claim.
    let changed = database!(tx, c, {
        sqlx::query("UPDATE agent_feed_items SET lifecycle_state=lifecycle_state WHERE tenant_id=$1 AND id=$2 AND lifecycle_state='APPROVED'").bind(tenant).bind(action).execute(c).await?.rows_affected()
    });
    if changed != 1 {
        return Err(Error::Invalid(
            "Persisted approval is missing, changed or revoked",
        ));
    }
    let raw: Option<(String,)> = database!(tx, c, {
        sqlx::query_as("SELECT CAST(proposed_action AS TEXT) FROM agent_feed_items WHERE tenant_id=$1 AND id=$2 AND lifecycle_state='APPROVED'").bind(tenant).bind(action).fetch_optional(c).await?
    });
    let payload: Value = serde_json::from_str(
        &raw.ok_or(Error::Invalid("Approved payload is unavailable"))?
            .0,
    )
    .map_err(|_| Error::Invalid("Approved payload is invalid"))?;
    if let Transaction::Postgres(pg) = &mut tx {
        let decision:Option<(String,String,Option<String>,Option<sqlx::types::Json<Value>>)> = sqlx::query_as(
            "SELECT decision_state,dispatch_status,job_id,dispatch_payload FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=$2 FOR UPDATE")
            .bind(tenant).bind(action).fetch_optional(&mut **pg).await?;
        match (decision, admitted) {
            (Some((state, status, job, snapshot)), Some((admitted_job, admitted_payload)))
                if state == "APPROVED"
                    && status == "ATTEMPTING"
                    && job.as_deref() == Some(admitted_job)
                    && snapshot.as_ref().is_some_and(|snapshot| {
                        &snapshot.0 == admitted_payload
                            && snapshot["action_id"] == action
                            && snapshot["tenant_id"] == tenant
                            && snapshot["payload"] == payload
                    }) => {}
            (None, None) => {}
            _ => {
                return Err(Error::Invalid(
                    "Existing owner dispatch is cancelled, changed or not admitted; reconcile before retrying",
                ));
            }
        }
    } else if admitted.is_some() {
        return Err(Error::Invalid(
            "Queue admission requires canonical PostgreSQL authority",
        ));
    }
    if !matches!(
        payload["feature_type"].as_str(),
        Some("ambassador_reply" | "instagram_dm")
    ) || payload["delivery_authority"] != "owner_review_required"
    {
        return Err(Error::Invalid(
            "Refresh this reply for review with a canonical provider binding",
        ));
    }
    let reviewed: Option<(String,)> = database!(tx, c, {
        sqlx::query_as("SELECT action_type FROM agent_action_requests WHERE tenant_id=$1 AND id=$2 AND action_type='HIGH'").bind(tenant).bind(action).fetch_optional(c).await?
    });
    if reviewed.is_none() {
        return Err(Error::Invalid(
            "Verified owner-reviewed reply required; automatic policy is unavailable",
        ));
    }
    let id = text(&payload, "inbox_message_id")?;
    let body = payload
        .get("generated_response")
        .or_else(|| payload.get("draft_reply"))
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or(Error::Invalid("Approved reply body is missing"))?;
    let hash = format!("{:x}", Sha256::digest(payload.to_string().as_bytes()));
    let previous: Option<(String, String, Option<String>, String)> = database!(tx, c, {
        sqlx::query_as("SELECT payload_hash,state,provider_message_id,detail FROM department_message_dispatches WHERE tenant_id=$1 AND action_id=$2").bind(tenant).bind(action).fetch_optional(c).await?
    });
    if let Some((saved, state, provider_message_id, detail)) = previous {
        if saved != hash {
            return Err(Error::Invalid(
                "Attempted content changed; reconcile before retrying",
            ));
        }
        tx.commit().await?;
        return Ok(DeliveryReceipt {
            state,
            provider_message_id,
            detail,
        });
    }
    let row = inbox(&mut tx, tenant, id).await?;
    if row.source.as_deref().unwrap_or_default() != payload["source"].as_str().unwrap_or_default()
        || row.sender_id.as_deref().unwrap_or_default()
            != payload["sender_id"].as_str().unwrap_or_default()
    {
        return Err(Error::Invalid(
            "Approved recipient or channel no longer matches canonical inbox",
        ));
    }
    let recipient = payload["sender_id"].as_str().unwrap_or_default();
    let binding: Option<Binding> = serde_json::from_value(payload["delivery_binding"].clone())
        .map_err(|_| Error::Invalid("Approved provider binding is invalid"))?;
    let creds = if let Some(binding) = &binding {
        credential(
            &mut tx,
            tenant,
            &binding.source,
            Some(&binding.credential_id),
        )
        .await?
    } else {
        None
    };
    let superseded = payload["manual_intent_revision"].as_i64().unwrap_or(0)
        != manual_intent_revision(&mut tx, tenant, id).await?;
    let historical = row.status.as_deref().is_some_and(historical_state);
    let terminal = row.terminal;
    let ready = !superseded
        && !historical
        && !terminal
        && match (&binding, &creds) {
            (Some(binding), Some(creds)) => {
                creds.binding(payload["source"].as_str().unwrap_or_default()) == *binding
                    && creds.configured(binding)
                    && (valid_recipient(&binding.source, recipient)
                        || (binding.integration_id == "whatsapp_cloud_api"
                            && binding.source == "whatsapp"
                            && crate::integrations::meta::client::is_business_scoped_recipient(
                                recipient,
                            )))
            }
            _ => false,
        };
    let state = if ready || historical {
        "unknown"
    } else {
        "blocked"
    };
    let detail = if superseded {
        "A later manual inbox action retired this approval; obtain a fresh review"
    } else if terminal {
        "Inbox was closed, dismissed or paused; no send attempted"
    } else if historical {
        "Prior send outcome is unverified; reconcile before another attempt"
    } else if ready {
        "Provider outcome not yet confirmed; reconcile before retrying"
    } else {
        "Approved provider account, credentials or recipient unavailable; no send attempted"
    };
    let now = chrono::Utc::now().timestamp();
    let inserted = database!(tx, c, {
        sqlx::query("INSERT INTO department_message_dispatches(tenant_id,action_id,inbox_message_id,inbox_kind,payload_hash,provider_binding,state,detail,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$9) ON CONFLICT DO NOTHING")
            .bind(tenant).bind(action).bind(id).bind(&row.kind).bind(&hash).bind(payload["delivery_binding"].to_string()).bind(state).bind(detail).bind(now).execute(c).await?.rows_affected()
    });
    if inserted != 1 {
        return Err(Error::Invalid(
            "This inbox already has a fenced reply; reconcile before another send",
        ));
    }
    if !terminal && !superseded {
        set_inbox_state(
            &mut tx,
            tenant,
            id,
            if ready || historical {
                "delivery_unknown"
            } else {
                "delivery_blocked"
            },
            body,
        )
        .await?;
    }
    tx.commit().await?; // Durable replay fence exists before any HTTP side effect.
    if !ready {
        return Ok(DeliveryReceipt {
            state: state.into(),
            provider_message_id: None,
            detail: detail.into(),
        });
    }
    let result = provider
        .send(
            binding.as_ref().ok_or(Error::Invalid("Missing binding"))?,
            recipient,
            body,
            creds
                .as_ref()
                .ok_or(Error::Invalid("Missing credentials"))?,
        )
        .await;
    let (state, provider_message_id, detail, inbox_state) = match result {
        Ok(id) if !id.trim().is_empty() && id.len() <= 1024 => (
            "accepted",
            Some(id),
            "Provider accepted the message; delivery is unconfirmed",
            "provider_accepted",
        ),
        Err(SendFailure::Rejected) => (
            "rejected",
            None,
            "Provider rejected the message; no acceptance confirmed",
            "delivery_failed",
        ),
        Err(SendFailure::Blocked) => (
            "blocked",
            None,
            "Provider configuration invalid; no send attempted",
            "delivery_blocked",
        ),
        _ => (
            "unknown",
            None,
            "Provider outcome unknown; reconcile before retrying",
            "delivery_unknown",
        ),
    };
    let mut tx = store.begin(tenant).await?;
    let current = inbox(&mut tx, tenant, id).await?;
    let changed = database!(tx, c, {
        sqlx::query("UPDATE department_message_dispatches SET state=$1,provider_message_id=$2,detail=$3,updated_at=$4 WHERE tenant_id=$5 AND action_id=$6 AND state='unknown' AND payload_hash=$7")
            .bind(state).bind(&provider_message_id).bind(detail).bind(chrono::Utc::now().timestamp()).bind(tenant).bind(action).bind(hash).execute(c).await?.rows_affected()
    });
    if changed != 1 {
        return Err(Error::Invalid(
            "Provider result could not be correlated to its durable claim",
        ));
    }
    if !current.terminal {
        set_inbox_state(&mut tx, tenant, id, inbox_state, body).await?;
    }
    tx.commit().await?;
    Ok(DeliveryReceipt {
        state: state.into(),
        provider_message_id,
        detail: detail.into(),
    })
}

#[path = "manual_inbox.rs"]
pub mod manual_inbox;
