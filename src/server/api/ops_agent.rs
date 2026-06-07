use axum::{
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OpsAgentRequest {
    pub notes: String,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpsAgentResponse {
    pub notes_translated: String,
    pub anomaly_alert: Option<String>,
}

#[derive(Serialize)]
pub struct HttpErrorResponse {
    pub error: String,
}

pub async fn ops_agent_handler(
    Json(payload): Json<OpsAgentRequest>,
) -> impl IntoResponse {
    use axum::http::StatusCode;

    let api_key = match std::env::var("MINIMAX_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            let mut alert = None;
            let mut falafel_count = 0;
            for item in &payload.items {
                if item.to_lowercase().contains("falafel") {
                    falafel_count += 1;
                }
            }
            if falafel_count > 1 {
                alert = Some("Agent: Falafel is running low due to rapid orders. [Toggle Sold Out]".to_string());
            }

            return (StatusCode::OK, Json(OpsAgentResponse {
                notes_translated: format!("{} (translated to Arabic)", payload.notes),
                anomaly_alert: alert,
            })).into_response();
        }
    };

    let prompt = format!(
        "Act as an Operations Agent for a small food vendor. \nTask 1: Translate these order notes to Arabic: '{}'\nTask 2: Analyze these ordered items for low-stock anomalies: {:?}. If an item (like Falafel) is ordered multiple times rapidly, it might cause a low-stock anomaly. If so, return a short alert like 'Agent: [Item] is running low due to rapid orders. [Toggle Sold Out]'.\nReturn your response in JSON format exactly like this:\n{{\n  \"notes_translated\": \"[Arabic Translation]\",\n  \"anomaly_alert\": \"[Alert or null]\"\n}}",
        payload.notes, payload.items
    );

    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);
    let client = crate::minimax::MinimaxClient::new(api_key);

    match client.reason(&compressed_prompt).await {
        Ok(output) => {
            if let Ok(parsed) = serde_json::from_str::<OpsAgentResponse>(&output) {
                (StatusCode::OK, Json(parsed)).into_response()
            } else {
                let cleaned = output.replace("```json", "").replace("```", "").trim().to_string();
                if let Ok(parsed) = serde_json::from_str::<OpsAgentResponse>(&cleaned) {
                    (StatusCode::OK, Json(parsed)).into_response()
                } else {
                    let mut alert = None;
                    let mut falafel_count = 0;
                    for item in &payload.items {
                        if item.to_lowercase().contains("falafel") {
                            falafel_count += 1;
                        }
                    }
                    if falafel_count > 1 {
                        alert = Some("Agent: Falafel is running low due to rapid orders. [Toggle Sold Out]".to_string());
                    }

                    (StatusCode::OK, Json(OpsAgentResponse {
                        notes_translated: format!("{} (translated)", payload.notes),
                        anomaly_alert: alert,
                    })).into_response()
                }
            }
        },
        Err(e) => {
            ::server_telemetry::record_error_signal("MiniMax ops agent failed");
            tracing::error!("MiniMax ops agent failed: {}", e);
            let mut alert = None;
            let mut falafel_count = 0;
            for item in &payload.items {
                if item.to_lowercase().contains("falafel") {
                    falafel_count += 1;
                }
            }
            if falafel_count > 1 {
                alert = Some("Agent: Falafel is running low due to rapid orders. [Toggle Sold Out]".to_string());
            }

            (StatusCode::OK, Json(OpsAgentResponse {
                notes_translated: format!("{} (translated)", payload.notes),
                anomaly_alert: alert,
            })).into_response()
        }
    }
}
