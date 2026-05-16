use std::sync::Arc;
use crate::db::DB;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::orchestration::mesh::TeammateMesh;
use serde_json::json;
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JourneyPhase {
    New,
    OnboardingStarted,
    CoreInfoProvided,
    PaymentConnected,
    StoreLive,
    FirstSale,
    RetentionPhase,
}

impl std::fmt::Display for JourneyPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JourneyPhase::New => write!(f, "NEW"),
            JourneyPhase::OnboardingStarted => write!(f, "ONBOARDING_STARTED"),
            JourneyPhase::CoreInfoProvided => write!(f, "CORE_INFO_PROVIDED"),
            JourneyPhase::PaymentConnected => write!(f, "PAYMENT_CONNECTED"),
            JourneyPhase::StoreLive => write!(f, "STORE_LIVE"),
            JourneyPhase::FirstSale => write!(f, "FIRST_SALE"),
            JourneyPhase::RetentionPhase => write!(f, "RETENTION_PHASE"),
        }
    }
}

impl std::str::FromStr for JourneyPhase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEW" => Ok(JourneyPhase::New),
            "ONBOARDING_STARTED" => Ok(JourneyPhase::OnboardingStarted),
            "CORE_INFO_PROVIDED" => Ok(JourneyPhase::CoreInfoProvided),
            "PAYMENT_CONNECTED" => Ok(JourneyPhase::PaymentConnected),
            "STORE_LIVE" => Ok(JourneyPhase::StoreLive),
            "FIRST_SALE" => Ok(JourneyPhase::FirstSale),
            "RETENTION_PHASE" => Ok(JourneyPhase::RetentionPhase),
            _ => Err(format!("Unknown phase: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionEvent {
    StartOnboarding,
    ProvideCoreInfo,
    ConnectPayment,
    PublishStore,
    MakeFirstSale,
    CompleteFirstMonth,
}

pub struct JourneyManager {
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
}

impl JourneyManager {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    pub async fn get_current_phase(&self, tenant_id: &str) -> Result<JourneyPhase, String> {
        let row = sqlx::query("SELECT phase FROM tenant_journey WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(r) => r.try_get::<String, _>("phase").unwrap_or_else(|_| "NEW".to_string()).parse().map_err(|e: <JourneyPhase as std::str::FromStr>::Err| e.to_string()),
            None => Ok(JourneyPhase::New),
        }
    }

    pub async fn process_event(&self, tenant_id: &str, event: TransitionEvent) -> Result<JourneyPhase, String> {
        let current_phase = self.get_current_phase(tenant_id).await?;

        let new_phase = match (&current_phase, &event) {
            (JourneyPhase::New, TransitionEvent::StartOnboarding) => JourneyPhase::OnboardingStarted,
            (JourneyPhase::OnboardingStarted, TransitionEvent::ProvideCoreInfo) => JourneyPhase::CoreInfoProvided,
            (JourneyPhase::CoreInfoProvided, TransitionEvent::ConnectPayment) => JourneyPhase::PaymentConnected,
            (JourneyPhase::PaymentConnected, TransitionEvent::PublishStore) => JourneyPhase::StoreLive,
            (JourneyPhase::StoreLive, TransitionEvent::MakeFirstSale) => JourneyPhase::FirstSale,
            (JourneyPhase::FirstSale, TransitionEvent::CompleteFirstMonth) => JourneyPhase::RetentionPhase,
            _ => return Ok(current_phase.clone()), // Invalid transition, ignore
        };

        if current_phase != new_phase {
            self.update_phase(tenant_id, &current_phase, &new_phase).await?;
        }

        Ok(new_phase)
    }

    async fn update_phase(&self, tenant_id: &str, old_phase: &JourneyPhase, new_phase: &JourneyPhase) -> Result<(), String> {
        let now = Utc::now();
        let phase_str = new_phase.to_string();

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

        // UPSERT the current phase
        sqlx::query(
            r#"
            INSERT INTO tenant_journey (tenant_id, phase, updated_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (tenant_id) DO UPDATE SET phase = $2, updated_at = $3
            "#
        )
        .bind(tenant_id)
        .bind(&phase_str)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Record history
        sqlx::query(
            r#"
            INSERT INTO tenant_journey_history (tenant_id, from_phase, to_phase, occurred_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(tenant_id)
        .bind(old_phase.to_string())
        .bind(&phase_str)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        // Emit mesh event for AI orchestration
        let event_payload = json!({
            "tenant_id": tenant_id,
            "old_phase": old_phase.to_string(),
            "new_phase": phase_str,
            "timestamp": now.to_rfc3339()
        });

        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "journey_manager".to_string(),
            action: "PhaseTransition".to_string(),
            status: "success".to_string(),
            payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        let _ = self.mesh.publish("journey_events", serde_json::to_vec(&event).unwrap_or_default()).await;

        Ok(())
    }
}
