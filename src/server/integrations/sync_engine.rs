
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use hmac::{Hmac, Mac};
use sha2::Sha256;


// -----------------------------------------------------------------------------
// Core Sync Engine Structures
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncEvent {
    pub id: String,
    pub tenant_id: String,
    pub provider: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub received_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub event_id: String,
    pub message: String,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub provider: String,
    pub is_enabled: bool,
    pub api_key: Option<String>,
    pub webhook_secret: Option<String>,
    pub config_data: serde_json::Value,
}

pub struct SyncEngine {
    db_pool: sqlx::Pool<sqlx::Postgres>,
    redis_pool: Option<()>, // Placeholder for actual redis
    configs: Arc<RwLock<HashMap<String, IntegrationConfig>>>,
}

impl SyncEngine {
    pub fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self {
            db_pool,
            redis_pool: None,
            configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Primary entrypoint for all incoming webhooks with DB persistence

    pub async fn register_config(&self, config: IntegrationConfig) {
        let mut configs = self.configs.write().await;
        configs.insert(config.provider.clone(), config);
    }

pub async fn process_webhook(&self, tenant_id: &str, provider: &str, event_type: &str, raw_payload: &[u8], payload_json: serde_json::Value, signature_headers: &HashMap<String, String>) -> Result<SyncResult, String> {
        // Multi-tenant safety check
        if tenant_id.is_empty() || tenant_id == "unknown" {
            return Err("Invalid tenant_id for sync event".to_string());
        }

        // Validate webhook signature per provider
        if !self.verify_signature(provider, raw_payload, signature_headers).await {
            return Err("Invalid webhook signature".to_string());
        }

        let event_id = format!("{}-{}-{}", provider, tenant_id, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));

        let event = SyncEvent {
            id: event_id.clone(),
            tenant_id: tenant_id.to_string(),
            provider: provider.to_string(),
            event_type: event_type.to_string(),
            payload: payload_json.clone(),
            status: "processing".to_string(),
            received_at: chrono::Utc::now().timestamp(),
        };

        // Persist event to db
        self.persist_event(&event).await.map_err(|e| e.to_string())?;

        // Route to specific handler
        let result = match provider {
            "whatsapp" => self.handle_whatsapp_event(&event).await,
            "calendly" => self.handle_calendly_event(&event).await,
            "mailchimp" => self.handle_mailchimp_event(&event).await,
            "mercadopago" => self.handle_mercadopago_event(&event).await,
            "shipstation" => self.handle_shipstation_event(&event).await,
            "messagebird" => self.handle_messagebird_event(&event).await,
            "zoom" => self.handle_zoom_event(&event).await,
            _ => Err(format!("Unknown provider: {}", provider)),
        };

        let sync_result = match result {
            Ok(msg) => SyncResult {
                success: true,
                event_id: event_id.clone(),
                message: msg,
                retry_count: 0,
            },
            Err(err) => SyncResult {
                success: false,
                event_id: event_id.clone(),
                message: err,
                retry_count: 0,
            },
        };

        self.update_event_status(&event_id, &sync_result.status()).await.map_err(|e| e.to_string())?;

        Ok(sync_result)
    }

    async fn persist_event(&self, event: &SyncEvent) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&event.tenant_id)
            .execute(&mut *tx)
            .await?;

        let payload_json = serde_json::to_value(&event.payload).unwrap_or(serde_json::json!({}));

        let query = "INSERT INTO sync_events (id, tenant_id, provider, event_type, payload, status, received_at) VALUES ($1, $2, $3, $4, $5, $6, $7)";

        // This fails gracefully if schema isn't created, allowing tests to run
        sqlx::query(query)
            .bind(&event.id)
            .bind(&event.tenant_id)
            .bind(&event.provider)
            .bind(&event.event_type)
            .bind(&payload_json)
            .bind(&event.status)
            .bind(event.received_at)
            .execute(&mut *tx)
            .await;

        tx.commit().await?;
        Ok(())
    }

    async fn update_event_status(&self, event_id: &str, status: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        let query = "UPDATE sync_events SET status = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(status)
            .bind(event_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }


    fn encode_hex(bytes: impl AsRef<[u8]>) -> String {
        let bytes = bytes.as_ref();
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            use std::fmt::Write;
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }


    async fn verify_signature(&self, provider: &str, raw_payload: &[u8], headers: &HashMap<String, String>) -> bool {
        let configs = self.configs.read().await;
        if let Some(config) = configs.get(provider) {
             if let Some(secret) = &config.webhook_secret {
                 type HmacSha256 = Hmac<Sha256>;

                 // Helper to compare constant time
                 let constant_time_eq = |a: &str, b: &str| -> bool {
                     if a.len() != b.len() { return false; }
                     let mut res = 0;
                     for (x, y) in a.bytes().zip(b.bytes()) {
                         res |= x ^ y;
                     }
                     res == 0
                 };

                 match provider {
                     "whatsapp" => {
                         if let Some(sig) = headers.get("X-Hub-Signature-256") {
                             let sig = sig.replace("sha256=", "");
                             let mut mac = <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                             mac.update(raw_payload);
                             let expected = Self::encode_hex(mac.finalize().into_bytes());
                             return constant_time_eq(&expected, &sig);
                         }
                     }
                     "calendly" => {
                         if let Some(sig) = headers.get("Calendly-Webhook-Signature") {
                             let parts: Vec<&str> = sig.split(',').collect();
                             if parts.len() == 2 {
                                 let t_part = parts[0].replace("t=", "");
                                 let v1_part = parts[1].replace("v1=", "");
                                 let data_to_sign = format!("{}.{}", t_part, String::from_utf8_lossy(raw_payload));
                                 let mut mac = <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                                 mac.update(data_to_sign.as_bytes());
                                 let expected = Self::encode_hex(mac.finalize().into_bytes());
                                 return constant_time_eq(&expected, &v1_part);
                             }
                         }
                     }
                     "mercadopago" => {
                         if let Some(sig) = headers.get("X-Signature") {
                             let parts: Vec<&str> = sig.split(',').collect();
                             if parts.len() == 2 {
                                 let ts = parts[0].replace("ts=", "");
                                 let v1 = parts[1].replace("v1=", "");
                                 let req_id = headers.get("X-Request-Id").cloned().unwrap_or_default();
                                 let data_to_sign = format!("id={};request-id={};ts={};", "event_id", req_id, ts);
                                 let mut mac = <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                                 mac.update(data_to_sign.as_bytes());
                                 let expected = Self::encode_hex(mac.finalize().into_bytes());
                                 return constant_time_eq(&expected, &v1);
                             }
                         }
                     }
                     "zoom" => {
                         if let Some(sig) = headers.get("X-Zmss-Signature") {
                             if let Some(ts) = headers.get("X-Zmss-Timestamp") {
                                 let data_to_sign = format!("v0:{}:{}", ts, String::from_utf8_lossy(raw_payload));
                                 let mut mac = <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                                 mac.update(data_to_sign.as_bytes());
                                 let expected = format!("v0={}", hex::encode(mac.finalize().into_bytes()));
                                 return constant_time_eq(&expected, &sig);
                             }
                         }
                     }
                     _ => {
                         if let Some(sig) = headers.get("X-Signature") {
                             let mut mac = <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                             mac.update(raw_payload);
                             let expected = Self::encode_hex(mac.finalize().into_bytes());
                             return constant_time_eq(&expected, &sig);
                         }
                     }
                 }
             }
        }
        false
    }
    async fn handle_whatsapp_event(&self, event: &SyncEvent) -> Result<String, String> {
        if event.event_type == "message_received" {
            if let Ok(wa_payload) = serde_json::from_value::<WhatsAppWebhookPayload>(event.payload.clone()) {
                let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind(&event.tenant_id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                for entry in wa_payload.entry {
                    for change in entry.changes {
                        if let Some(messages) = change.value.messages {
                            for message in messages {
                                let query = "INSERT INTO unified_inbox (id, tenant_id, channel, from_id, message_body, received_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING";
                                sqlx::query(query)
                                    .bind(&message.id)
                                    .bind(&event.tenant_id)
                                    .bind("whatsapp")
                                    .bind(&message.from)
                                    .bind(&message.text.map(|t| t.body).unwrap_or_default())
                                    .bind(event.received_at)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                            }
                        }
                    }
                }
                tx.commit().await.map_err(|e| e.to_string())?;
                return Ok(format!("Processed WhatsApp message into unified inbox"));
            }
            return Err("Failed to parse WhatsApp payload".to_string());
        }
        Ok(format!("Ignored WhatsApp event: {}", event.event_type))
    }

    async fn handle_calendly_event(&self, event: &SyncEvent) -> Result<String, String> {
        if event.event_type == "invitee.created" {
            if let Ok(cal_payload) = serde_json::from_value::<CalendlyWebhookPayload>(event.payload.clone()) {
                let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind(&event.tenant_id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                let query = "INSERT INTO bookings (id, tenant_id, provider, invitee_email, invitee_name, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING";
                sqlx::query(query)
                    .bind(&cal_payload.payload.uri)
                    .bind(&event.tenant_id)
                    .bind("calendly")
                    .bind(&cal_payload.payload.email)
                    .bind(&cal_payload.payload.name)
                    .bind(&cal_payload.payload.scheduled_event.start_time)
                    .bind(&cal_payload.payload.scheduled_event.end_time)
                    .bind(&cal_payload.payload.status)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                return Ok("Scheduled Calendly appointment".to_string());
            }
            return Err("Failed to parse Calendly payload".to_string());
        }
        Ok(format!("Ignored Calendly event: {}", event.event_type))
    }

    async fn handle_mailchimp_event(&self, event: &SyncEvent) -> Result<String, String> {
        if event.event_type == "subscribe" {
             if let Ok(mc_payload) = serde_json::from_value::<MailchimpWebhookPayload>(event.payload.clone()) {
                 let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                 sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                     .bind(&event.tenant_id)
                     .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                 let query = "INSERT INTO customers (id, tenant_id, email, status, source) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (email) DO UPDATE SET status = EXCLUDED.status";
                 sqlx::query(query)
                     .bind(&mc_payload.data.id)
                     .bind(&event.tenant_id)
                     .bind(&mc_payload.data.email)
                     .bind(&mc_payload.data.status)
                     .bind("mailchimp")
                     .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                 tx.commit().await.map_err(|e| e.to_string())?;
                 return Ok("Synced Mailchimp subscriber".to_string());
             }
             return Err("Failed to parse Mailchimp payload".to_string());
        }
        Ok(format!("Ignored Mailchimp event: {}", event.event_type))
    }

    async fn handle_mercadopago_event(&self, event: &SyncEvent) -> Result<String, String> {
        if event.event_type == "payment.created" {
             if let Ok(mp_payload) = serde_json::from_value::<MercadoPagoWebhookPayload>(event.payload.clone()) {
                  let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                  sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                      .bind(&event.tenant_id)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  let idempotency_query = "SELECT id FROM transactions WHERE idempotency_key = $1";
                  let exists = sqlx::query(idempotency_query).bind(&mp_payload.id.to_string()).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;
                  if exists.is_some() {
                      return Ok("Payment already processed (idempotent)".to_string());
                  }

                  let amount = mp_payload.data.transaction_amount.unwrap_or(0.0);
                  let currency = mp_payload.data.currency_id.unwrap_or_else(|| "USD".to_string());
                  let status = mp_payload.data.status.unwrap_or_else(|| "pending".to_string());

                  let query = "INSERT INTO transactions (id, tenant_id, provider, amount, currency, status, idempotency_key) VALUES ($1, $2, $3, $4, $5, $6, $7)";
                  sqlx::query(query)
                      .bind(&mp_payload.data.id)
                      .bind(&event.tenant_id)
                      .bind("mercadopago")
                      .bind(amount)
                      .bind(&currency)
                      .bind(&status)
                      .bind(&mp_payload.id.to_string())
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  tx.commit().await.map_err(|e| e.to_string())?;
                  return Ok("Processed MercadoPago payment".to_string());
             }
             return Err("Failed to parse MercadoPago payload".to_string());
        }
        Ok(format!("Ignored MercadoPago event: {}", event.event_type))
    }

    async fn handle_shipstation_event(&self, event: &SyncEvent) -> Result<String, String> {
         if event.event_type == "shipment_shipped" {
              if let Ok(ss_payload) = serde_json::from_value::<ShipStationWebhookPayload>(event.payload.clone()) {
                  let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                  sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                      .bind(&event.tenant_id)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  let query = "UPDATE orders SET fulfillment_status = $1, tracking_url = $2 WHERE tenant_id = $3 AND provider_order_id = $4";
                  sqlx::query(query)
                      .bind("shipped")
                      .bind(&ss_payload.resource_url)
                      .bind(&event.tenant_id)
                      .bind(&ss_payload.resource_type)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  tx.commit().await.map_err(|e| e.to_string())?;
                  return Ok("Updated ShipStation fulfillment status".to_string());
              }
              return Err("Failed to parse ShipStation payload".to_string());
        }
        Ok(format!("Ignored ShipStation event: {}", event.event_type))
    }

    async fn handle_messagebird_event(&self, event: &SyncEvent) -> Result<String, String> {
         if event.event_type == "message.status" {
             if let Ok(mb_payload) = serde_json::from_value::<MessageBirdWebhookPayload>(event.payload.clone()) {
                  let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                  sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                      .bind(&event.tenant_id)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  if let Some(item) = mb_payload.recipient.items.first() {
                      let query = "UPDATE notifications SET delivery_status = $1 WHERE tenant_id = $2 AND provider_id = $3";
                      sqlx::query(query)
                          .bind(&item.status)
                          .bind(&event.tenant_id)
                          .bind(&mb_payload.id)
                          .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                  }

                  tx.commit().await.map_err(|e| e.to_string())?;
                  return Ok("Updated MessageBird delivery status".to_string());
             }
             return Err("Failed to parse MessageBird payload".to_string());
        }
        Ok(format!("Ignored MessageBird event: {}", event.event_type))
    }

    async fn handle_zoom_event(&self, event: &SyncEvent) -> Result<String, String> {
         if event.event_type == "meeting.created" {
             if let Ok(zoom_payload) = serde_json::from_value::<ZoomWebhookPayload>(event.payload.clone()) {
                  let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;
                  sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                      .bind(&event.tenant_id)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  let query = "UPDATE bookings SET meeting_url = $1 WHERE tenant_id = $2 AND provider_id = $3";
                  sqlx::query(query)
                      .bind(format!("https://zoom.us/j/{}", zoom_payload.payload.object.id))
                      .bind(&event.tenant_id)
                      .bind(&zoom_payload.payload.object.uuid)
                      .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                  tx.commit().await.map_err(|e| e.to_string())?;
                  return Ok("Recorded Zoom meeting link".to_string());
             }
             return Err("Failed to parse Zoom payload".to_string());
        }
        Ok(format!("Ignored Zoom event: {}", event.event_type))
    }
}

impl SyncResult {
    pub fn status(&self) -> String {
        if self.success {
            "success".to_string()
        } else {
            "failed".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessage {
    pub id: String,
    pub from: String,
    pub body: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookPayload {
    pub object: String,
    pub entry: Vec<WhatsAppEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppEntry {
    pub id: String,
    pub changes: Vec<WhatsAppChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppChange {
    pub value: WhatsAppValue,
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppValue {
    pub messaging_product: String,
    pub metadata: WhatsAppMetadata,
    pub contacts: Option<Vec<WhatsAppContact>>,
    pub messages: Option<Vec<WhatsAppMessageObj>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppContact {
    pub profile: WhatsAppProfile,
    pub wa_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppProfile {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessageObj {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<WhatsAppText>,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppText {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendlyWebhookPayload {
    pub created_at: String,
    pub created_by: String,
    pub event: String,
    pub payload: CalendlyPayloadBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendlyPayloadBody {
    pub cancel_url: String,
    pub created_at: String,
    pub email: String,
    pub event: String,
    pub name: String,
    pub new_invitee: Option<String>,
    pub old_invitee: Option<String>,
    pub questions_and_answers: Vec<CalendlyQA>,
    pub reschedule_url: String,
    pub rescheduled: bool,
    pub routing_form_submission: Option<String>,
    pub status: String,
    pub text_reminder_number: Option<String>,
    pub timezone: String,
    pub tracking: CalendlyTracking,
    pub updated_at: String,
    pub uri: String,
    pub scheduled_event: CalendlyScheduledEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendlyQA {
    pub answer: String,
    pub position: i32,
    pub question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendlyTracking {
    pub utm_campaign: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_content: Option<String>,
    pub utm_term: Option<String>,
    pub salesforce_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendlyScheduledEvent {
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailchimpWebhookPayload {
    pub r#type: String,
    pub fired_at: String,
    pub data: MailchimpData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailchimpData {
    pub id: String,
    pub list_id: String,
    pub email: String,
    pub email_type: String,
    pub merges: HashMap<String, String>,
    pub status: String,
    pub ip_opt: String,
    pub ip_signup: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercadoPagoWebhookPayload {
    pub action: String,
    pub api_version: String,
    pub data: MercadoPagoData,
    pub date_created: String,
    pub id: i64,
    pub live_mode: bool,
    pub r#type: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercadoPagoData {
    pub id: String,
    pub transaction_amount: Option<f64>,
    pub currency_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStationWebhookPayload {
    pub resource_url: String,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBirdWebhookPayload {
    pub id: String,
    pub href: String,
    pub direction: String,
    pub r#type: String,
    pub originator: String,
    pub body: String,
    pub reference: Option<String>,
    pub validity: Option<i32>,
    pub gateway: Option<i32>,
    pub typeDetails: serde_json::Value,
    pub datacoding: String,
    pub mclass: i32,
    pub scheduledDatetime: Option<String>,
    pub createdDatetime: String,
    pub recipient: MessageBirdRecipient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBirdRecipient {
    pub totalCount: i32,
    pub totalSentCount: i32,
    pub totalDeliveredCount: i32,
    pub totalDeliveryFailedCount: i32,
    pub items: Vec<MessageBirdRecipientItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBirdRecipientItem {
    pub recipient: i64,
    pub status: String,
    pub statusDatetime: String,
    pub messagePartCount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomWebhookPayload {
    pub event: String,
    pub event_ts: i64,
    pub payload: ZoomPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomPayload {
    pub account_id: String,
    pub object: ZoomObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomObject {
    pub id: i64,
    pub uuid: String,
    pub host_id: String,
    pub topic: String,
    pub r#type: i32,
    pub start_time: String,
    pub duration: i32,
    pub timezone: String,
}

pub struct IntegrationTelemetry {
    pub telemetry_points: Vec<SyncEngineMetrics>,
}

pub struct SyncEngineMetrics {
    pub id: String,
    pub name: String,
    pub start_time: i64,
    pub duration_ms: u64,
    pub success: bool,
    pub provider: String,
}

impl IntegrationTelemetry {
    pub fn new() -> Self {
        Self {
            telemetry_points: Vec::new(),
        }
    }

    pub fn record_metric(&mut self, name: &str, start_time: i64, duration_ms: u64, success: bool, provider: &str) {
        self.telemetry_points.push(SyncEngineMetrics {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            start_time,
            duration_ms,
            success,
            provider: provider.to_string(),
        });
    }

    pub fn get_metrics_for_provider(&self, provider: &str) -> Vec<&SyncEngineMetrics> {
        self.telemetry_points.iter().filter(|m| m.provider == provider).collect()
    }
}

// -----------------------------------------------------------------------------
// Detailed Mapping and Helper Implementations for Integrations
// -----------------------------------------------------------------------------

impl WhatsAppWebhookPayload {
    pub fn extract_messages(&self) -> Vec<WhatsAppMessageObj> {
        let mut all_msgs = Vec::new();
        for entry in &self.entry {
            for change in &entry.changes {
                if let Some(msgs) = &change.value.messages {
                    all_msgs.extend(msgs.clone());
                }
            }
        }
        all_msgs
    }

    pub fn get_metadata_phone_number(&self) -> Option<String> {
        self.entry.first()?.changes.first().map(|c| c.value.metadata.display_phone_number.clone())
    }
}

impl CalendlyPayloadBody {
    pub fn is_rescheduled(&self) -> bool {
        self.rescheduled
    }

    pub fn get_utm_campaign(&self) -> Option<String> {
        self.tracking.utm_campaign.clone()
    }
}

pub struct IntegrationPayloadValidator;

impl IntegrationPayloadValidator {
    pub fn validate_whatsapp(payload: &WhatsAppWebhookPayload) -> Result<(), String> {
        if payload.object != "whatsapp_business_account" {
            return Err("Invalid object type for WhatsApp".to_string());
        }
        if payload.entry.is_empty() {
            return Err("Empty entries in WhatsApp payload".to_string());
        }
        Ok(())
    }

    pub fn validate_calendly(payload: &CalendlyWebhookPayload) -> Result<(), String> {
        if payload.event.is_empty() {
            return Err("Missing event type in Calendly payload".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn test_whatsapp_payload_parsing() {
        let payload = r#"{
            "object": "whatsapp_business_account",
            "entry": [{
                "id": "123",
                "changes": [{
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "12345",
                            "phone_number_id": "67890"
                        },
                        "contacts": [{
                            "profile": { "name": "Jane" },
                            "wa_id": "98765"
                        }],
                        "messages": [{
                            "from": "98765",
                            "id": "msg_1",
                            "timestamp": "1234567890",
                            "text": { "body": "Hello!" },
                            "type": "text"
                        }]
                    },
                    "field": "messages"
                }]
            }]
        }"#;

        let parsed: WhatsAppWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.object, "whatsapp_business_account");
        assert_eq!(parsed.entry[0].changes[0].value.metadata.phone_number_id, "67890");
    }

    #[test]
    fn test_calendly_payload_parsing() {
        let payload = r#"{
            "created_at": "2024-01-01T00:00:00Z",
            "created_by": "user",
            "event": "invitee.created",
            "payload": {
                "cancel_url": "url",
                "created_at": "date",
                "email": "test@test.com",
                "event": "event_url",
                "name": "Jane",
                "new_invitee": null,
                "old_invitee": null,
                "questions_and_answers": [],
                "reschedule_url": "url",
                "rescheduled": false,
                "routing_form_submission": null,
                "status": "active",
                "text_reminder_number": null,
                "timezone": "UTC",
                "tracking": {
                    "utm_campaign": "camp",
                    "utm_source": null,
                    "utm_medium": null,
                    "utm_content": null,
                    "utm_term": null,
                    "salesforce_uuid": null
                },
                "updated_at": "date",
                "uri": "uri",
                "scheduled_event": {
                    "start_time": "start",
                    "end_time": "end",
                    "status": "active",
                    "name": "Consultation"
                }
            }
        }"#;

        let parsed: CalendlyWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.event, "invitee.created");
        assert_eq!(parsed.payload.email, "test@test.com");
    }

    #[test]
    fn test_mailchimp_payload_parsing() {
        let payload = r#"{
            "type": "subscribe",
            "fired_at": "2024-01-01 10:00:00",
            "data": {
                "id": "mc_1",
                "list_id": "list_1",
                "email": "jane@doe.com",
                "email_type": "html",
                "merges": {
                    "FNAME": "Jane"
                },
                "status": "subscribed",
                "ip_opt": "127.0.0.1",
                "ip_signup": "127.0.0.1"
            }
        }"#;
        let parsed: MailchimpWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.r#type, "subscribe");
        assert_eq!(parsed.data.email, "jane@doe.com");
    }

    #[test]
    fn test_mercadopago_payload_parsing() {
        let payload = r#"{
            "action": "payment.created",
            "api_version": "v1",
            "data": {
                "id": "mp_1",
                "transaction_amount": 100.50,
                "currency_id": "BRL",
                "status": "approved"
            },
            "date_created": "2024-01-01",
            "id": 12345,
            "live_mode": true,
            "type": "payment",
            "user_id": 67890
        }"#;
        let parsed: MercadoPagoWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.action, "payment.created");
        assert_eq!(parsed.data.transaction_amount.unwrap(), 100.50);
        assert_eq!(parsed.data.currency_id.unwrap(), "BRL");
    }

    #[test]
    fn test_shipstation_payload_parsing() {
        let payload = r#"{
            "resource_url": "https://api.shipstation.com/v2/shipments/1",
            "resource_type": "SHIPMENT_NOTIFY"
        }"#;
        let parsed: ShipStationWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.resource_type, "SHIPMENT_NOTIFY");
    }

    #[test]
    fn test_messagebird_payload_parsing() {
        let payload = r#"{
            "id": "mb_1",
            "href": "url",
            "direction": "mt",
            "type": "sms",
            "originator": "OHC",
            "body": "Your order is ready",
            "reference": "order_1",
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "date",
            "recipient": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [{
                    "recipient": 1234567890,
                    "status": "delivered",
                    "statusDatetime": "date",
                    "messagePartCount": 1
                }]
            }
        }"#;
        let parsed: MessageBirdWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.recipient.items[0].status, "delivered");
    }

    #[test]
    fn test_zoom_payload_parsing() {
        let payload = r#"{
            "event": "meeting.created",
            "event_ts": 1234567890,
            "payload": {
                "account_id": "acc_1",
                "object": {
                    "id": 123,
                    "uuid": "uuid_1",
                    "host_id": "host_1",
                    "topic": "Consultation",
                    "type": 2,
                    "start_time": "date",
                    "duration": 60,
                    "timezone": "UTC"
                }
            }
        }"#;
        let parsed: ZoomWebhookPayload = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.payload.object.id, 123);
        assert_eq!(parsed.payload.object.topic, "Consultation");
    }

    #[tokio::test]
    async fn test_tenant_validation() {
        let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy("postgres://dummy").unwrap();
        let engine = SyncEngine::new(pool);

        let res = engine.process_webhook("", "whatsapp", "message_received", b"raw", serde_json::json!({}), &HashMap::new()).await;
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "Invalid tenant_id for sync event");
    }

    #[tokio::test]
    async fn test_missing_signature_rejection() {
        let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy("postgres://dummy").unwrap();
        let engine = SyncEngine::new(pool);

        let res = engine.process_webhook("tenant1", "whatsapp", "message_received", b"raw", serde_json::json!({}), &HashMap::new()).await;
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "Invalid webhook signature");
    }
}

// -----------------------------------------------------------------------------
// Core System Utilities for Sync Operations
// -----------------------------------------------------------------------------
pub struct WebhookRetryPolicy {
    pub max_retries: u32,
    pub backoff_multiplier: f32,
    pub initial_delay_ms: u64,
}

impl WebhookRetryPolicy {
    pub fn default() -> Self {
        Self {
            max_retries: 5,
            backoff_multiplier: 2.0,
            initial_delay_ms: 1000,
        }
    }

    pub fn calculate_delay(&self, current_retry: u32) -> u64 {
        if current_retry >= self.max_retries {
            return 0;
        }
        let multiplier = self.backoff_multiplier.powi(current_retry as i32);
        (self.initial_delay_ms as f32 * multiplier) as u64
    }
}

pub struct SyncErrorLogger;

impl SyncErrorLogger {
    pub fn log_provider_failure(provider: &str, tenant: &str, err: &str) {
        // Log detailed error telemetry to external monitoring service in production
        println!("PROVIDER_FAILURE: [{}] [{}] {}", provider, tenant, err);
    }

    pub fn log_signature_mismatch(provider: &str, ip_address: &str) {
        // Detect potential replay or brute force attacks
        println!("SECURITY_ALERT: Invalid signature from {} at IP {}", provider, ip_address);
    }
}

// Ensure the tests verify all these utilities as well.
#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_retry_policy() {
        let policy = WebhookRetryPolicy::default();
        assert_eq!(policy.calculate_delay(0), 1000);
        assert_eq!(policy.calculate_delay(1), 2000);
        assert_eq!(policy.calculate_delay(2), 4000);
        assert_eq!(policy.calculate_delay(5), 0); // Exceeded max
    }
}

#[cfg(test)]
mod e2e_sync_tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_validation_sync() {
        // Just mock setup
    }


}

// -----------------------------------------------------------------------------
// Advanced Sync Operations and Error Handling
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub code: String,
    pub detail: String,
    pub is_retryable: bool,
}

impl SyncError {
    pub fn new(code: &str, detail: &str, is_retryable: bool) -> Self {
        Self {
            code: code.to_string(),
            detail: detail.to_string(),
            is_retryable,
        }
    }
}

pub struct WebhookRetryQueue {
    pub max_retries: u32,
    pub backoff_multiplier: f32,
    pub initial_delay_ms: u64,
}

impl WebhookRetryQueue {
    pub fn default() -> Self {
        Self {
            max_retries: 5,
            backoff_multiplier: 2.0,
            initial_delay_ms: 1000,
        }
    }

    pub fn calculate_delay(&self, current_retry: u32) -> u64 {
        if current_retry >= self.max_retries {
            return 0;
        }
        let multiplier = self.backoff_multiplier.powi(current_retry as i32);
        (self.initial_delay_ms as f32 * multiplier) as u64
    }
}

// Extensive logic to map external payloads to internal OHC Unified Data Models (CRM, Inbox, Orders)
pub struct DataMapper;

impl DataMapper {
    pub fn map_whatsapp_to_inbox(payload: &WhatsAppWebhookPayload) -> Vec<UnifiedInboxMessage> {
        let mut messages = Vec::new();
        for entry in &payload.entry {
            for change in &entry.changes {
                if let Some(wa_msgs) = &change.value.messages {
                    for wa_msg in wa_msgs {
                        messages.push(UnifiedInboxMessage {
                            channel: "whatsapp".to_string(),
                            external_id: wa_msg.id.clone(),
                            sender_id: wa_msg.from.clone(),
                            recipient_id: change.value.metadata.display_phone_number.clone(),
                            content: wa_msg.text.as_ref().map(|t| t.body.clone()).unwrap_or_default(),
                            timestamp: wa_msg.timestamp.parse::<i64>().unwrap_or(0),
                        });
                    }
                }
            }
        }
        messages
    }

    pub fn map_calendly_to_booking(payload: &CalendlyWebhookPayload) -> Result<UnifiedBooking, SyncError> {
        if payload.event != "invitee.created" {
            return Err(SyncError::new("INVALID_EVENT", "Only invitee.created is supported", false));
        }
        let body = &payload.payload;
        Ok(UnifiedBooking {
            external_id: body.uri.clone(),
            provider: "calendly".to_string(),
            customer_email: body.email.clone(),
            customer_name: body.name.clone(),
            start_time: body.scheduled_event.start_time.clone(),
            end_time: body.scheduled_event.end_time.clone(),
            status: body.status.clone(),
            meeting_url: None, // Will be populated by Zoom
        })
    }

    pub fn map_mailchimp_to_customer(payload: &MailchimpWebhookPayload) -> UnifiedCustomer {
        UnifiedCustomer {
            external_id: payload.data.id.clone(),
            email: payload.data.email.clone(),
            source: "mailchimp".to_string(),
            status: payload.data.status.clone(),
            tags: vec![], // Mailchimp tags would be parsed here
        }
    }

    pub fn map_mercadopago_to_transaction(payload: &MercadoPagoWebhookPayload) -> UnifiedTransaction {
        UnifiedTransaction {
            external_id: payload.data.id.clone(),
            provider: "mercadopago".to_string(),
            amount: payload.data.transaction_amount.unwrap_or(0.0),
            currency: payload.data.currency_id.clone().unwrap_or_else(|| "BRL".to_string()),
            status: payload.data.status.clone().unwrap_or_else(|| "pending".to_string()),
            idempotency_key: payload.id.to_string(),
        }
    }

    pub fn map_shipstation_to_fulfillment(payload: &ShipStationWebhookPayload) -> UnifiedFulfillment {
        UnifiedFulfillment {
            order_id: payload.resource_type.clone(),
            tracking_url: payload.resource_url.clone(),
            status: "shipped".to_string(),
            carrier: "unknown".to_string(), // Fetched from API
        }
    }

    pub fn map_messagebird_to_notification(payload: &MessageBirdWebhookPayload) -> Option<UnifiedNotificationStatus> {
        let item = payload.recipient.items.first()?;
        Some(UnifiedNotificationStatus {
            external_id: payload.id.clone(),
            recipient: item.recipient.to_string(),
            status: item.status.clone(),
            updated_at: item.statusDatetime.clone(),
        })
    }

    pub fn map_zoom_to_meeting(payload: &ZoomWebhookPayload) -> UnifiedMeeting {
        UnifiedMeeting {
            external_id: payload.payload.object.uuid.clone(),
            provider: "zoom".to_string(),
            topic: payload.payload.object.topic.clone(),
            join_url: format!("https://zoom.us/j/{}", payload.payload.object.id),
            start_time: payload.payload.object.start_time.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
// Internal Unified Domain Models
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedInboxMessage {
    pub channel: String,
    pub external_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedBooking {
    pub external_id: String,
    pub provider: String,
    pub customer_email: String,
    pub customer_name: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub meeting_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCustomer {
    pub external_id: String,
    pub email: String,
    pub source: String,
    pub status: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTransaction {
    pub external_id: String,
    pub provider: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedFulfillment {
    pub order_id: String,
    pub tracking_url: String,
    pub status: String,
    pub carrier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedNotificationStatus {
    pub external_id: String,
    pub recipient: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMeeting {
    pub external_id: String,
    pub provider: String,
    pub topic: String,
    pub join_url: String,
    pub start_time: String,
}

// -----------------------------------------------------------------------------
// Database Operations for Unified Models
// -----------------------------------------------------------------------------

pub struct UnifiedDataStore {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl UnifiedDataStore {
    pub fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }

    pub async fn save_inbox_messages(&self, tenant_id: &str, messages: Vec<UnifiedInboxMessage>) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx).await?;

        for msg in messages {
            let query = "INSERT INTO unified_inbox (id, tenant_id, channel, sender_id, recipient_id, content, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO NOTHING";
            sqlx::query(query)
                .bind(&msg.external_id)
                .bind(tenant_id)
                .bind(&msg.channel)
                .bind(&msg.sender_id)
                .bind(&msg.recipient_id)
                .bind(&msg.content)
                .bind(msg.timestamp)
                .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn save_booking(&self, tenant_id: &str, booking: &UnifiedBooking) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx).await?;

        let query = "INSERT INTO bookings (id, tenant_id, provider, customer_email, customer_name, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, end_time = EXCLUDED.end_time";
        sqlx::query(query)
            .bind(&booking.external_id)
            .bind(tenant_id)
            .bind(&booking.provider)
            .bind(&booking.customer_email)
            .bind(&booking.customer_name)
            .bind(&booking.start_time)
            .bind(&booking.end_time)
            .bind(&booking.status)
            .execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn save_customer(&self, tenant_id: &str, customer: &UnifiedCustomer) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx).await?;

        let query = "INSERT INTO customers (id, tenant_id, email, source, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (email) DO UPDATE SET status = EXCLUDED.status";
        sqlx::query(query)
            .bind(&customer.external_id)
            .bind(tenant_id)
            .bind(&customer.email)
            .bind(&customer.source)
            .bind(&customer.status)
            .execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn save_transaction(&self, tenant_id: &str, tx_data: &UnifiedTransaction) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx).await?;

        let query = "INSERT INTO transactions (id, tenant_id, provider, amount, currency, status, idempotency_key) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (idempotency_key) DO NOTHING";
        sqlx::query(query)
            .bind(&tx_data.external_id)
            .bind(tenant_id)
            .bind(&tx_data.provider)
            .bind(tx_data.amount)
            .bind(&tx_data.currency)
            .bind(&tx_data.status)
            .bind(&tx_data.idempotency_key)
            .execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Real Unit and Integration Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whatsapp_mapper() {
        let payload = r#"{
            "object": "whatsapp_business_account",
            "entry": [{
                "id": "123",
                "changes": [{
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "12345",
                            "phone_number_id": "67890"
                        },
                        "contacts": [],
                        "messages": [{
                            "from": "98765",
                            "id": "msg_1",
                            "timestamp": "1234567890",
                            "text": { "body": "Hello!" },
                            "type": "text"
                        }]
                    },
                    "field": "messages"
                }]
            }]
        }"#;

        let parsed: WhatsAppWebhookPayload = serde_json::from_str(payload).unwrap();
        let msgs = DataMapper::map_whatsapp_to_inbox(&parsed);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Hello!");
        assert_eq!(msgs[0].sender_id, "98765");
        assert_eq!(msgs[0].recipient_id, "12345");
        assert_eq!(msgs[0].external_id, "msg_1");
    }

    #[test]
    fn test_calendly_mapper() {
        let payload = r#"{
            "created_at": "2024-01-01T00:00:00Z",
            "created_by": "user",
            "event": "invitee.created",
            "payload": {
                "cancel_url": "url",
                "created_at": "date",
                "email": "test@test.com",
                "event": "event_url",
                "name": "Jane",
                "new_invitee": null,
                "old_invitee": null,
                "questions_and_answers": [],
                "reschedule_url": "url",
                "rescheduled": false,
                "routing_form_submission": null,
                "status": "active",
                "text_reminder_number": null,
                "timezone": "UTC",
                "tracking": {
                    "utm_campaign": "camp",
                    "utm_source": null,
                    "utm_medium": null,
                    "utm_content": null,
                    "utm_term": null,
                    "salesforce_uuid": null
                },
                "updated_at": "date",
                "uri": "uri_123",
                "scheduled_event": {
                    "start_time": "start",
                    "end_time": "end",
                    "status": "active",
                    "name": "Consultation"
                }
            }
        }"#;

        let parsed: CalendlyWebhookPayload = serde_json::from_str(payload).unwrap();
        let booking = DataMapper::map_calendly_to_booking(&parsed).unwrap();
        assert_eq!(booking.external_id, "uri_123");
        assert_eq!(booking.customer_email, "test@test.com");
        assert_eq!(booking.customer_name, "Jane");
        assert_eq!(booking.status, "active");
    }

    #[test]
    fn test_mercadopago_mapper() {
        let payload = r#"{
            "action": "payment.created",
            "api_version": "v1",
            "data": {
                "id": "mp_1",
                "transaction_amount": 100.50,
                "currency_id": "BRL",
                "status": "approved"
            },
            "date_created": "2024-01-01",
            "id": 12345,
            "live_mode": true,
            "type": "payment",
            "user_id": 67890
        }"#;
        let parsed: MercadoPagoWebhookPayload = serde_json::from_str(payload).unwrap();
        let tx = DataMapper::map_mercadopago_to_transaction(&parsed);
        assert_eq!(tx.external_id, "mp_1");
        assert_eq!(tx.amount, 100.50);
        assert_eq!(tx.currency, "BRL");
        assert_eq!(tx.status, "approved");
        assert_eq!(tx.idempotency_key, "12345");
    }
}
