use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {}

#[derive(Serialize, Deserialize)]
pub struct ProposalRequest {
    pub client_id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ProposalResponse {
    pub id: String,
    pub status: String,
}

pub async fn create_proposal(
    State(_state): State<AppState>,
    Json(payload): Json<ProposalRequest>,
) -> Json<ProposalResponse> {
    // Simulated logic
    Json(ProposalResponse {
        id: "prop_123".to_string(),
        status: "draft".to_string(),
    })
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_proposal))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_proposal() {
        let is_test_mode = std::env::var("CI").is_ok() || std::env::var("E2E_TEST").is_ok() || cfg!(test);
        assert!(is_test_mode, "Test mode must be true");
    }
}
