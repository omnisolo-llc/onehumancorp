
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram};
use uuid::Uuid;
use ::server_ohc::orchestration::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub subject: String,
    pub body_html: String,
    pub status: CampaignStatus,
    pub metrics: CampaignMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Completed,
    Failed,
}

impl ToString for CampaignStatus {
    fn to_string(&self) -> String {
        match self {
            CampaignStatus::Draft => "DRAFT".to_string(),
            CampaignStatus::Scheduled => "SCHEDULED".to_string(),
            CampaignStatus::Sending => "SENDING".to_string(),
            CampaignStatus::Completed => "COMPLETED".to_string(),
            CampaignStatus::Failed => "FAILED".to_string(),
        }
    }
}

impl From<&str> for CampaignStatus {
    fn from(s: &str) -> Self {
        match s {
            "DRAFT" => CampaignStatus::Draft,
            "SCHEDULED" => CampaignStatus::Scheduled,
            "SENDING" => CampaignStatus::Sending,
            "COMPLETED" => CampaignStatus::Completed,
            "FAILED" => CampaignStatus::Failed,
            _ => CampaignStatus::Draft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignMetrics {
    pub total_sent: i32,
    pub total_delivered: i32,
    pub total_opened: i32,
    pub total_clicked: i32,
    pub total_bounced: i32,
    pub total_complained: i32,
}

pub struct CampaignService {
    pool: PgPool,
    campaigns_created: Counter<u64>,
    campaigns_sent: Counter<u64>,
    send_duration: Histogram<f64>,
}

impl CampaignService {
    pub fn new(pool: PgPool) -> Self {
        let meter = global::meter("ohc.growth.campaigns");
        Self {
            pool,
            campaigns_created: meter.u64_counter("campaigns.created").build(),
            campaigns_sent: meter.u64_counter("campaigns.sent").build(),
            send_duration: meter.f64_histogram("campaigns.send_duration").build(),
        }
    }
}
