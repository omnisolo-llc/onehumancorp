use serde::{Deserialize, Serialize};
use crate::tools::tenant::TenantContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Escalation {
    pub id: String,
    pub tenant_id: String,
    pub location_id: Option<String>,
    pub related_task_id: Option<String>,
    pub summary: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEscalationRequest {
    pub location_id: Option<String>,
    pub related_task_id: Option<String>,
    pub summary: String,
}

pub async fn create_escalation(ctx: &TenantContext, req: CreateEscalationRequest) -> Result<Escalation, String> {
    let escalation = Escalation {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: ctx.tenant_id.clone(),
        location_id: req.location_id,
        related_task_id: req.related_task_id,
        summary: req.summary,
        status: "open".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    Ok(escalation)
}
