use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendlyEventType {
    pub id: String,
    pub name: String,
    pub description_plain: String,
}

pub struct CalendlyClient {
    pub api_token: String,
}

impl CalendlyClient {
    pub fn new(api_token: String) -> Self {
        CalendlyClient { api_token }
    }

    pub async fn list_event_types(&self, tenant_id: &str) -> Result<Vec<CalendlyEventType>, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "calendly_list_event_types",
            0.15
        ).await;
        Ok(vec![
            CalendlyEventType {
                id: "event_type_1".to_string(),
                name: "30-min Consultation".to_string(),
                description_plain: "Discuss your project with us.".to_string(),
            }
        ])
    }
}
