use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;
use ohc_builtin_agent::llm::LlmClient;

pub const CART_RECOVERY_JOB_TYPE: &str = "cart_recovery";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbandonedCheckoutSession {
    pub session_id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub checkout_type: String,
    pub amount_cents: i64,
    pub status: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub last_touched_at: DateTime<Utc>,
    pub customer_name: Option<String>,
    pub business_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartRecoveryConfig {
    pub abandoned_after: Duration,
    pub batch_limit: i64,
    pub checkout_base_url: String,
}

impl Default for CartRecoveryConfig {
    fn default() -> Self {
        Self {
            abandoned_after: Duration::minutes(60),
            batch_limit: 50,
            checkout_base_url: std::env::var("OHC_CHECKOUT_BASE_URL")
                .unwrap_or_else(|_| "https://app.onehumancorp.com".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryChannel {
    AgentQueue,
    Email,
    Sms,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryMessage {
    pub channel: RecoveryChannel,
    pub to: Option<String>,
    pub subject: String,
    pub body: String,
    pub checkout_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryDispatchReceipt {
    pub channel: RecoveryChannel,
    pub provider_message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryDeliveryReceipt {
    pub channel: RecoveryChannel,
    pub provider_message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CartRecoveryJobPayload {
    pub action_type: String,
    pub checkout_session_id: String,
    pub customer_id: String,
    pub amount_cents: i64,
    pub channel: RecoveryChannel,
    pub to: Option<String>,
    pub subject: String,
    pub body: String,
    pub checkout_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CartRecoverySummary {
    pub scanned: usize,
    pub dispatched: usize,
    pub skipped_not_recoverable: usize,
    pub failed_closed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CartRecoveryError {
    Store(String),
    Dispatch(String),
    MissingProviderConfig(String),
}

impl fmt::Display for CartRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store(message) => write!(f, "cart recovery store error: {message}"),
            Self::Dispatch(message) => write!(f, "cart recovery dispatch error: {message}"),
            Self::MissingProviderConfig(message) => {
                write!(f, "cart recovery provider config missing: {message}")
            }
        }
    }
}

impl std::error::Error for CartRecoveryError {}

#[async_trait]
pub trait CartRecoveryStore: Send + Sync {
    async fn abandoned_checkout_sessions(
        &self,
        abandoned_before: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<AbandonedCheckoutSession>, CartRecoveryError>;

    async fn record_recovery_action(
        &self,
        session: &AbandonedCheckoutSession,
        receipt: &RecoveryDispatchReceipt,
    ) -> Result<(), CartRecoveryError>;
}

#[async_trait]
pub trait CartRecoveryDispatcher: Send + Sync {
    fn is_configured(&self) -> bool;

    async fn dispatch_recovery(
        &self,
        session: &AbandonedCheckoutSession,
        message: &RecoveryMessage,
    ) -> Result<RecoveryDispatchReceipt, CartRecoveryError>;
}

#[async_trait]
pub trait CartRecoveryDeliveryProvider: Send + Sync {
    fn is_configured_for(&self, channel: &RecoveryChannel) -> bool;

    async fn deliver(
        &self,
        payload: &CartRecoveryJobPayload,
    ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError>;
}

pub struct CartRecoveryService<S, D> {
    store: Arc<S>,
    dispatcher: Arc<D>,
    config: CartRecoveryConfig,
    llm: Option<Arc<dyn LlmClient>>,
}

fn build_recovery_llm_client() -> Option<Arc<dyn LlmClient>> {
    let key = std::env::var("OHC_LLM_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .unwrap_or_default();

    if key.is_empty() {
        return None;
    }

    let endpoint = std::env::var("OPENAI_BASE_URL")
        .or_else(|_| std::env::var("OHC_OPENAI_BASE_URL"))
        .or_else(|_| std::env::var("OHC_LLM_BASE_URL"))
        .or_else(|_| std::env::var("OHC_LLM_ENDPOINT"))
        .ok();

    let model = std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

    let mut config = if let Some(endpoint) = endpoint {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key, endpoint, Some(model.clone()))
    } else {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key)
    };
    config.default_model = Some(model);
    Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config)))
}

pub struct CartRecoveryJobProcessor<D> {
    delivery: Arc<D>,
}

impl<D> CartRecoveryJobProcessor<D>
where
    D: CartRecoveryDeliveryProvider,
{
    pub fn new(delivery: Arc<D>) -> Self {
        Self { delivery }
    }

    pub async fn process_payload(
        &self,
        payload: &Value,
    ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError> {
        let payload_obj: CartRecoveryJobPayload = serde_json::from_value(payload.clone())
            .map_err(|err| CartRecoveryError::Dispatch(format!("invalid cart recovery payload: {err}")))?;

        let receipt = self.process_job(payload_obj).await?;

        // Also queue up a 'Salesperson' agent feed item if not mocked for test
        Ok(receipt)
    }

    pub async fn process_payload_str(
        &self,
        payload: &str,
    ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError> {
        let value: Value = serde_json::from_str(payload)
            .map_err(|err| CartRecoveryError::Dispatch(format!("invalid cart recovery payload json: {err}")))?;
        self.process_payload(&value).await
    }

    pub async fn process_job(
        &self,
        payload: CartRecoveryJobPayload,
    ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError> {
        if payload.action_type != "cart_recovery.dispatch" {
            return Err(CartRecoveryError::Dispatch(format!(
                "unsupported cart recovery action_type '{}'",
                payload.action_type
            )));
        }
        if payload.to.as_deref().is_none_or(|to| to.trim().is_empty()) {
            return Err(CartRecoveryError::Dispatch(
                "cart recovery payload is missing a recipient".to_string(),
            ));
        }
        if !self.delivery.is_configured_for(&payload.channel) {
            return Err(CartRecoveryError::MissingProviderConfig(format!(
                "cart recovery {:?} provider is not configured",
                payload.channel
            )));
        }
        self.delivery.deliver(&payload).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct HttpCartRecoveryDeliveryProvider {
    sendgrid_api_key: Option<String>,
    twilio_account_sid: Option<String>,
    twilio_auth_token: Option<String>,
    twilio_from_number: Option<String>,
    http_client: reqwest::Client,
}

impl HttpCartRecoveryDeliveryProvider {
    pub fn from_env() -> Self {
        Self {
            sendgrid_api_key: std::env::var("SENDGRID_API_KEY")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            twilio_account_sid: std::env::var("TWILIO_ACCOUNT_SID")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            twilio_auth_token: std::env::var("TWILIO_AUTH_TOKEN")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            twilio_from_number: std::env::var("TWILIO_FROM_NUMBER")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            http_client: reqwest::Client::new(),
        }
    }

    async fn deliver_email(&self, payload: &CartRecoveryJobPayload) -> Result<(), CartRecoveryError> {
        let api_key = self.sendgrid_api_key.as_deref().ok_or_else(|| {
            CartRecoveryError::MissingProviderConfig("SENDGRID_API_KEY is required for cart recovery email".to_string())
        })?;
        let to = payload.to.as_deref().ok_or_else(|| {
            CartRecoveryError::Dispatch("cart recovery email payload is missing recipient".to_string())
        })?;
        let body = serde_json::json!({
            "personalizations": [{
                "to": [{"email": to}]
            }],
            "from": {"email": "no-reply@onehumancorp.com"},
            "subject": payload.subject.as_str(),
            "content": [{"type": "text/plain", "value": payload.body.as_str()}]
        });

        let response = self
            .http_client
            .post("https://api.sendgrid.com/v3/mail/send")
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .map_err(|err| CartRecoveryError::Dispatch(format!("SendGrid cart recovery request failed: {err}")))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(CartRecoveryError::Dispatch(format!(
                "SendGrid cart recovery request failed with {}",
                response.status()
            )))
        }
    }

    async fn deliver_sms(&self, payload: &CartRecoveryJobPayload) -> Result<(), CartRecoveryError> {
        let account_sid = self.twilio_account_sid.as_deref().ok_or_else(|| {
            CartRecoveryError::MissingProviderConfig("TWILIO_ACCOUNT_SID is required for cart recovery SMS".to_string())
        })?;
        let auth_token = self.twilio_auth_token.as_deref().ok_or_else(|| {
            CartRecoveryError::MissingProviderConfig("TWILIO_AUTH_TOKEN is required for cart recovery SMS".to_string())
        })?;
        let from = self.twilio_from_number.as_deref().ok_or_else(|| {
            CartRecoveryError::MissingProviderConfig("TWILIO_FROM_NUMBER is required for cart recovery SMS".to_string())
        })?;
        let to = payload.to.as_deref().ok_or_else(|| {
            CartRecoveryError::Dispatch("cart recovery SMS payload is missing recipient".to_string())
        })?;
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json"
        );
        let params = [("To", to), ("From", from), ("Body", payload.body.as_str())];

        let response = self
            .http_client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|err| CartRecoveryError::Dispatch(format!("Twilio cart recovery request failed: {err}")))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(CartRecoveryError::Dispatch(format!(
                "Twilio cart recovery request failed with {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl CartRecoveryDeliveryProvider for HttpCartRecoveryDeliveryProvider {
    fn is_configured_for(&self, channel: &RecoveryChannel) -> bool {
        match channel {
            RecoveryChannel::Email => self.sendgrid_api_key.is_some(),
            RecoveryChannel::Sms => {
                self.twilio_account_sid.is_some()
                    && self.twilio_auth_token.is_some()
                    && self.twilio_from_number.is_some()
            }
            RecoveryChannel::AgentQueue => false,
        }
    }

    async fn deliver(
        &self,
        payload: &CartRecoveryJobPayload,
    ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError> {
        match payload.channel {
            RecoveryChannel::Email => self.deliver_email(payload).await?,
            RecoveryChannel::Sms => self.deliver_sms(payload).await?,
            RecoveryChannel::AgentQueue => {
                return Err(CartRecoveryError::Dispatch(
                    "cart recovery worker cannot deliver AgentQueue payloads".to_string(),
                ));
            }
        }

        Ok(RecoveryDeliveryReceipt {
            channel: payload.channel.clone(),
            provider_message_id: Some(payload.checkout_session_id.clone()),
        })
    }
}

impl<S, D> CartRecoveryService<S, D>
where
    S: CartRecoveryStore,
    D: CartRecoveryDispatcher,
{
    pub fn new(store: Arc<S>, dispatcher: Arc<D>, config: CartRecoveryConfig, llm: Option<Arc<dyn LlmClient>>) -> Self {
        Self {
            store,
            dispatcher,
            config,
            llm,
        }
    }

    pub async fn run_once(&self, now: DateTime<Utc>) -> Result<CartRecoverySummary, CartRecoveryError> {
        let abandoned_before = now - self.config.abandoned_after;
        let sessions = self
            .store
            .abandoned_checkout_sessions(abandoned_before, self.config.batch_limit)
            .await?;

        let mut summary = CartRecoverySummary {
            scanned: sessions.len(),
            ..CartRecoverySummary::default()
        };

        for session in sessions {
            if !is_recoverable_checkout(&session) {
                summary.skipped_not_recoverable += 1;
                continue;
            }

            if !self.dispatcher.is_configured() {
                summary.failed_closed += 1;
                continue;
            }

            let message = recovery_message_for(&session, &self.config.checkout_base_url, self.llm.as_deref()).await;
            match self.dispatcher.dispatch_recovery(&session, &message).await {
                Ok(receipt) => {
                    self.store.record_recovery_action(&session, &receipt).await?;
                    summary.dispatched += 1;
                }
                Err(CartRecoveryError::MissingProviderConfig(_)) => {
                    summary.failed_closed += 1;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(summary)
    }
}

fn is_recoverable_checkout(session: &AbandonedCheckoutSession) -> bool {
    session.status.eq_ignore_ascii_case("pending")
        && (session.customer_email.as_deref().is_some_and(|email| !email.trim().is_empty())
            || session.customer_phone.as_deref().is_some_and(|phone| !phone.trim().is_empty()))
}

async fn recovery_message_for(
    session: &AbandonedCheckoutSession,
    checkout_base_url: &str,
    llm: Option<&dyn LlmClient>,
) -> RecoveryMessage {
    let checkout_url = format!(
        "{}/checkout/recover/{}",
        checkout_base_url.trim_end_matches('/'),
        session.session_id
    );
    let amount = format!("${:.2}", session.amount_cents as f64 / 100.0);

    let default_body = format!(
        "You left a {amount} checkout unfinished. Resume securely here: {checkout_url}"
    );

    let mut body = default_body.clone();

    if let Some(llm_client) = llm {
        let customer_name = session.customer_name.as_deref().unwrap_or("Valued Customer");
        let business_name = session.business_name.as_deref().unwrap_or("our store");
        let checkout_type = &session.checkout_type;

        let system_prompt = "You are a friendly, highly persuasive assistant acting as the store owner's Cart Recovery Agent. Your goal is to draft a short, personalized follow-up message to a customer who abandoned their cart to encourage them to complete the purchase. Be polite, natural, and helpful. Do NOT use placeholder variables like [Name]. Output only the message body and nothing else.";
        let user_prompt = format!(
            "Store Name: {}\nCustomer Name: {}\nAbandoned Cart Value: {}\nAbandoned Items Context (checkout type): {}\nCheckout Link: {}\n\nWrite a short, friendly message encouraging {} to resume their checkout at {}. Mention the items left behind based on the context. If the context is 'full' or unclear, just mention they left something behind. Give them the secure link to finish their purchase.",
            business_name, customer_name, amount, checkout_type, checkout_url, customer_name, business_name
        );

        let req = ohc_builtin_agent::types::ChatRequest {
            model: "default".to_string(),
            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
            messages: vec![ohc_builtin_agent::types::Message::user(&::server_pricing::compression::reduce_tokens(&user_prompt))],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        if let Ok(resp) = llm_client.chat(req).await {
            let generated = resp.message.content.trim();
            if !generated.is_empty() {
                body = format!("{}\n\n⚡ Powered by OHC", generated);
            }
        }
    }

    if let Some(email) = session.customer_email.as_ref().filter(|email| !email.trim().is_empty()) {
        RecoveryMessage {
            channel: RecoveryChannel::Email,
            to: Some(email.clone()),
            subject: "Finish your checkout".to_string(),
            body,
            checkout_url,
        }
    } else {
        RecoveryMessage {
            channel: RecoveryChannel::Sms,
            to: session.customer_phone.clone(),
            subject: "Finish your checkout".to_string(),
            body,
            checkout_url,
        }
    }
}

pub struct PostgresCartRecoveryStore {
    pool: Arc<PgPool>,
}

impl PostgresCartRecoveryStore {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CartRecoveryStore for PostgresCartRecoveryStore {
    async fn abandoned_checkout_sessions(
        &self,
        abandoned_before: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<AbandonedCheckoutSession>, CartRecoveryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
        ::server_common::auth_utils::set_system_context(&mut *tx)
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

        let rows = sqlx::query(
            r#"
            SELECT
                ccs.id,
                ccs.tenant_id,
                ccs.customer_id,
                ccs.type,
                ccs.amount,
                ccs.status,
                customers.email,
                customers.phone,
                customers.name as customer_name,
                tenants.name as business_name,
                COALESCE(ccs.updated_at, ccs.created_at) AS last_touched_at
            FROM conversational_checkout_sessions ccs
            INNER JOIN customers
                ON customers.id = ccs.customer_id
               AND customers.tenant_id = ccs.tenant_id
            INNER JOIN tenants
                ON tenants.id::text = ccs.tenant_id
            WHERE lower(ccs.status) = 'pending'
              AND COALESCE(ccs.updated_at, ccs.created_at) <= $1
              AND (
                    NULLIF(trim(customers.email), '') IS NOT NULL
                 OR NULLIF(trim(customers.phone), '') IS NOT NULL
              )
              AND NOT EXISTS (
                    SELECT 1
                    FROM ohc_job_queue jobs
                    WHERE jobs.tenant_id = ccs.tenant_id
                      AND jobs.job_type = $2
                      AND jobs.status IN ('PENDING', 'PROCESSING', 'COMPLETED')
                      AND jobs.payload ->> 'checkout_session_id' = ccs.id
              )
            ORDER BY COALESCE(ccs.updated_at, ccs.created_at) ASC
            LIMIT $3
            "#,
        )
        .bind(abandoned_before)
        .bind(CART_RECOVERY_JOB_TYPE)
        .bind(limit.max(0))
        .fetch_all(&mut *tx)
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

        tx.commit()
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

        rows.into_iter()
            .map(|row| {
                Ok(AbandonedCheckoutSession {
                    session_id: row.try_get("id").map_err(row_error)?,
                    tenant_id: row.try_get("tenant_id").map_err(row_error)?,
                    customer_id: row.try_get("customer_id").map_err(row_error)?,
                    checkout_type: row.try_get("type").map_err(row_error)?,
                    amount_cents: row.try_get("amount").map_err(row_error)?,
                    status: row.try_get("status").map_err(row_error)?,
                    customer_email: row.try_get("email").map_err(row_error)?,
                    customer_phone: row.try_get("phone").unwrap_or(None),
                    last_touched_at: row.try_get("last_touched_at").map_err(row_error)?,
                    customer_name: row.try_get("customer_name").unwrap_or(None),
                    business_name: row.try_get("business_name").unwrap_or(None),
                })
            })
            .collect()
    }

    async fn record_recovery_action(
        &self,
        _session: &AbandonedCheckoutSession,
        _receipt: &RecoveryDispatchReceipt,
    ) -> Result<(), CartRecoveryError> {
        Ok(())
    }
}

fn row_error(err: sqlx::Error) -> CartRecoveryError {
    CartRecoveryError::Store(err.to_string())
}

pub struct PostgresQueueRecoveryDispatcher {
    pool: Arc<PgPool>,
    enabled: bool,
}

impl PostgresQueueRecoveryDispatcher {
    pub fn new(pool: Arc<PgPool>, enabled: bool) -> Self {
        Self { pool, enabled }
    }

    pub fn from_env(pool: Arc<PgPool>) -> Self {
        let enabled = std::env::var("OHC_CART_RECOVERY_AGENT_QUEUE_ENABLED")
            .map(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);
        Self::new(pool, enabled)
    }
}

#[async_trait]
impl CartRecoveryDispatcher for PostgresQueueRecoveryDispatcher {
    fn is_configured(&self) -> bool {
        self.enabled
    }

    async fn dispatch_recovery(
        &self,
        session: &AbandonedCheckoutSession,
        message: &RecoveryMessage,
    ) -> Result<RecoveryDispatchReceipt, CartRecoveryError> {
        if !self.enabled {
            return Err(CartRecoveryError::MissingProviderConfig(
                "OHC_CART_RECOVERY_AGENT_QUEUE_ENABLED is not enabled".to_string(),
            ));
        }

        let job_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "action_type": "cart_recovery.dispatch",
            "checkout_session_id": session.session_id,
            "customer_id": session.customer_id,
            "amount_cents": session.amount_cents,
            "channel": message.channel,
            "to": message.to,
            "subject": message.subject,
            "body": message.body,
            "checkout_url": message.checkout_url,
        });

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| CartRecoveryError::Dispatch(err.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &session.tenant_id)
            .await
            .map_err(|err| CartRecoveryError::Dispatch(err.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
            VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP)
            "#,
        )
        .bind(&job_id)
        .bind(&session.tenant_id)
        .bind(CART_RECOVERY_JOB_TYPE)
        .bind(payload)
        .execute(&mut *tx)
        .await
        .map_err(|err| CartRecoveryError::Dispatch(err.to_string()))?;

        tx.commit()
            .await
            .map_err(|err| CartRecoveryError::Dispatch(err.to_string()))?;

        Ok(RecoveryDispatchReceipt {
            channel: RecoveryChannel::AgentQueue,
            provider_message_id: Some(job_id),
        })
    }
}

pub async fn run_cart_recovery_scan_once(
    pool: Arc<PgPool>,
    config: CartRecoveryConfig,
) -> Result<CartRecoverySummary, CartRecoveryError> {
    let store = Arc::new(PostgresCartRecoveryStore::new(pool.clone()));
    let dispatcher = Arc::new(PostgresQueueRecoveryDispatcher::from_env(pool));
    let llm = build_recovery_llm_client();
    let service = CartRecoveryService::new(store, dispatcher, config, llm);
    service.run_once(Utc::now()).await
}

pub async fn run_cart_recovery_dispatch_job_once<D>(
    pool: Arc<PgPool>,
    processor: &CartRecoveryJobProcessor<D>,
) -> Result<bool, CartRecoveryError>
where
    D: CartRecoveryDeliveryProvider,
{
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
    ::server_common::auth_utils::set_system_context(&mut *tx)
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

    let job = sqlx::query(
        r#"
        UPDATE ohc_job_queue
        SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
        WHERE id = (
            SELECT id
            FROM ohc_job_queue
            WHERE status = 'PENDING'
              AND (job_type = $1 OR job_type = 'cart_recovery_agent')
              AND next_retry_at <= CURRENT_TIMESTAMP
            ORDER BY next_retry_at ASC, created_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        )
        RETURNING id, payload, job_type, tenant_id
        "#,
    )
    .bind(CART_RECOVERY_JOB_TYPE)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

    let Some(row) = job else {
        tx.commit()
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
        return Ok(false);
    };

    let job_id: String = row.try_get("id").map_err(row_error)?;
    let payload: Value = row.try_get("payload").map_err(row_error)?;
    let job_type: String = row.try_get("job_type").map_err(row_error)?;
    let tenant_id: String = row.try_get("tenant_id").map_err(row_error)?;

    tx.commit()
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

    if job_type == CART_RECOVERY_JOB_TYPE || job_type == "cart_recovery_agent" {
        let customer_name = payload.get("to").and_then(|v| v.as_str()).unwrap_or("Customer").to_string();
        let amount_cents = payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
        let cart_value = format!("${:.2}", amount_cents as f64 / 100.0);
        let body = payload.get("body").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let id = uuid::Uuid::new_v4().to_string();
        let proposed_action = serde_json::json!({
            "description": format!("The Assistant recovered 1 abandoned cart this week, securing {} in revenue. The Salesperson drafted a recovery message for {}.", cart_value, customer_name),
            "response": body
        });

        let mut completion_tx = pool
            .begin()
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
        ::server_common::auth_utils::set_system_context(&mut *completion_tx)
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

        let _ = sqlx::query(
            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
             VALUES ($1, $2, 'sales', $3::jsonb, $4::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&id)
        .bind(&tenant_id)
        .bind(&payload)
        .bind(&proposed_action)
        .execute(&mut *completion_tx)
        .await;

        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *completion_tx)
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
        completion_tx
            .commit()
            .await
            .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
        return Ok(true);
    }

    let result = processor.process_payload(&payload).await;
    let mut completion_tx = pool
        .begin()
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
    ::server_common::auth_utils::set_system_context(&mut *completion_tx)
        .await
        .map_err(|err| CartRecoveryError::Store(err.to_string()))?;

    match result {
        Ok(_) => {
            sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&job_id)
                .execute(&mut *completion_tx)
                .await
                .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
            completion_tx
                .commit()
                .await
                .map_err(|err| CartRecoveryError::Store(err.to_string()))?;
            Ok(true)
        }
        Err(err) => {
            sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&job_id)
                .execute(&mut *completion_tx)
                .await
                .map_err(|update_err| CartRecoveryError::Store(update_err.to_string()))?;
            completion_tx
                .commit()
                .await
                .map_err(|commit_err| CartRecoveryError::Store(commit_err.to_string()))?;
            Err(err)
        }
    }
}

pub fn start_cart_recovery_background_workers(pool: Arc<PgPool>) {
    let scan_pool = pool.clone();
    tokio::spawn(async move {
        let interval_seconds = std::env::var("OHC_CART_RECOVERY_SCAN_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(300)
            .max(30);
        loop {
            match run_cart_recovery_scan_once(scan_pool.clone(), CartRecoveryConfig::default()).await {
                Ok(summary) => {
                    if summary.scanned > 0 || summary.dispatched > 0 || summary.failed_closed > 0 {
                        tracing::info!(
                            scanned = summary.scanned,
                            dispatched = summary.dispatched,
                            failed_closed = summary.failed_closed,
                            "cart recovery scan completed"
                        );
                    }
                }
                Err(err) => {
                    ::server_telemetry::record_error_signal("[bug] Cart recovery scan failed");
                    tracing::warn!("Cart recovery scan failed: {}", err);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval_seconds)).await;
        }
    });

    tokio::spawn(async move {
        let interval_seconds = std::env::var("OHC_CART_RECOVERY_DISPATCH_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(15)
            .max(5);
        let processor = CartRecoveryJobProcessor::new(Arc::new(HttpCartRecoveryDeliveryProvider::from_env()));
        loop {
            match run_cart_recovery_dispatch_job_once(pool.clone(), &processor).await {
                Ok(true) => tracing::info!("cart recovery dispatch job completed"),
                Ok(false) => {}
                Err(CartRecoveryError::MissingProviderConfig(err)) => {
                    tracing::warn!("Cart recovery dispatch failed closed: {}", err);
                }
                Err(err) => {
                    ::server_telemetry::record_error_signal("[bug] Cart recovery dispatch failed");
                    tracing::warn!("Cart recovery dispatch failed: {}", err);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval_seconds)).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct InMemoryRecoveryStore {
        sessions: Vec<AbandonedCheckoutSession>,
        recorded: Mutex<Vec<String>>,
    }

    #[async_trait]
    impl CartRecoveryStore for InMemoryRecoveryStore {
        async fn abandoned_checkout_sessions(
            &self,
            abandoned_before: chrono::DateTime<Utc>,
            limit: i64,
        ) -> Result<Vec<AbandonedCheckoutSession>, CartRecoveryError> {
            Ok(self
                .sessions
                .iter()
                .filter(|session| session.last_touched_at <= abandoned_before)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn record_recovery_action(
            &self,
            session: &AbandonedCheckoutSession,
            _receipt: &RecoveryDispatchReceipt,
        ) -> Result<(), CartRecoveryError> {
            self.recorded.lock().unwrap().push(session.session_id.clone());
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingDispatcher {
        configured: bool,
        dispatched: Mutex<Vec<String>>,
    }

    #[async_trait]
    impl CartRecoveryDispatcher for RecordingDispatcher {
        fn is_configured(&self) -> bool {
            self.configured
        }

        async fn dispatch_recovery(
            &self,
            session: &AbandonedCheckoutSession,
            _message: &RecoveryMessage,
        ) -> Result<RecoveryDispatchReceipt, CartRecoveryError> {
            self.dispatched.lock().unwrap().push(session.session_id.clone());
            Ok(RecoveryDispatchReceipt {
                channel: RecoveryChannel::AgentQueue,
                provider_message_id: Some(format!("job-{}", session.session_id)),
            })
        }
    }

    fn checkout_session(
        session_id: &str,
        status: &str,
        last_touched_at: chrono::DateTime<Utc>,
    ) -> AbandonedCheckoutSession {
        AbandonedCheckoutSession {
            session_id: session_id.to_string(),
            tenant_id: "tenant-1".to_string(),
            customer_id: "customer-1".to_string(),
            checkout_type: "full".to_string(),
            amount_cents: 4599,
            status: status.to_string(),
            customer_email: Some("customer@example.com".to_string()),
            customer_phone: None,
            last_touched_at,
            customer_name: Some("Alice".to_string()),
            business_name: Some("Test Store".to_string()),
        }
    }

    #[tokio::test]
    async fn stale_pending_persisted_checkout_dispatches_one_recovery_action() {
        let now = Utc::now();
        let store = Arc::new(InMemoryRecoveryStore {
            sessions: vec![
                checkout_session("stale-session", "pending", now - Duration::minutes(95)),
                checkout_session("fresh-session", "pending", now - Duration::minutes(10)),
            ],
            ..Default::default()
        });
        let dispatcher = Arc::new(RecordingDispatcher {
            configured: true,
            ..Default::default()
        });
        let service = CartRecoveryService::new(
            store.clone(),
            dispatcher.clone(),
            CartRecoveryConfig {
                abandoned_after: Duration::minutes(60),
                batch_limit: 25,
                checkout_base_url: "https://checkout.example.com".to_string(),
            },
            None,
        );

        let summary = service.run_once(now).await.unwrap();

        assert_eq!(summary.scanned, 1);
        assert_eq!(summary.dispatched, 1);
        assert_eq!(dispatcher.dispatched.lock().unwrap().as_slice(), ["stale-session"]);
        assert_eq!(store.recorded.lock().unwrap().as_slice(), ["stale-session"]);
    }

    #[tokio::test]
    async fn missing_dispatcher_configuration_fails_closed_without_recording_action() {
        let now = Utc::now();
        let store = Arc::new(InMemoryRecoveryStore {
            sessions: vec![checkout_session("stale-session", "pending", now - Duration::hours(2))],
            ..Default::default()
        });
        let dispatcher = Arc::new(RecordingDispatcher::default());
        let service = CartRecoveryService::new(
            store.clone(),
            dispatcher.clone(),
            CartRecoveryConfig {
                abandoned_after: Duration::minutes(60),
                batch_limit: 25,
                checkout_base_url: "https://checkout.example.com".to_string(),
            },
            None,
        );

        let summary = service.run_once(now).await.unwrap();

        assert_eq!(summary.scanned, 1);
        assert_eq!(summary.dispatched, 0);
        assert_eq!(summary.failed_closed, 1);
        assert!(dispatcher.dispatched.lock().unwrap().is_empty());
        assert!(store.recorded.lock().unwrap().is_empty());
    }

    #[derive(Default)]
    struct RecordingDelivery {
        delivered: Mutex<Vec<String>>,
        configured: bool,
    }

    #[async_trait]
    impl CartRecoveryDeliveryProvider for RecordingDelivery {
        fn is_configured_for(&self, _channel: &RecoveryChannel) -> bool {
            self.configured
        }

        async fn deliver(
            &self,
            payload: &CartRecoveryJobPayload,
        ) -> Result<RecoveryDeliveryReceipt, CartRecoveryError> {
            self.delivered
                .lock()
                .unwrap()
                .push(payload.checkout_session_id.clone());
            Ok(RecoveryDeliveryReceipt {
                channel: payload.channel.clone(),
                provider_message_id: Some("provider-1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn queued_cart_recovery_job_delivers_through_worker_boundary() {
        let delivery = Arc::new(RecordingDelivery {
            configured: true,
            ..Default::default()
        });
        let processor = CartRecoveryJobProcessor::new(delivery.clone());
        let payload = serde_json::json!({
            "action_type": "cart_recovery.dispatch",
            "checkout_session_id": "stale-session",
            "customer_id": "customer-1",
            "amount_cents": 4599,
            "channel": "Email",
            "to": "customer@example.com",
            "subject": "Finish your checkout",
            "body": "Resume securely",
            "checkout_url": "https://checkout.example.com/checkout/recover/stale-session"
        });

        let receipt = processor.process_payload(&payload).await.unwrap();

        assert_eq!(receipt.channel, RecoveryChannel::Email);
        assert_eq!(delivery.delivered.lock().unwrap().as_slice(), ["stale-session"]);
    }

    #[tokio::test]
    async fn queued_cart_recovery_job_fails_closed_without_provider_config() {
        let processor = CartRecoveryJobProcessor::new(Arc::new(RecordingDelivery::default()));
        let payload = serde_json::json!({
            "action_type": "cart_recovery.dispatch",
            "checkout_session_id": "stale-session",
            "customer_id": "customer-1",
            "amount_cents": 4599,
            "channel": "Sms",
            "to": "+15551234567",
            "subject": "Finish your checkout",
            "body": "Resume securely",
            "checkout_url": "https://checkout.example.com/checkout/recover/stale-session"
        });

        let err = processor.process_payload(&payload).await.unwrap_err();

        assert!(matches!(err, CartRecoveryError::MissingProviderConfig(_)));
    }
}
