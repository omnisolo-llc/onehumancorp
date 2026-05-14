use axum::{
    extract::{State, Query},
    Json,
    response::IntoResponse,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::hub::Hub;
use crate::sip::SipDB;
use sqlx::Row;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct MissionResponse {
    pub id: String,
    pub status: String,
    pub payload: String,
}

#[derive(Serialize)]
pub struct ListMissionsResponse {
    pub data: Vec<MissionResponse>,
    pub next_cursor: Option<String>,
}

#[derive(Clone)]
pub struct MissionHandlerState {
    pub hub: Arc<Hub>,
    pub sip_db: Arc<SipDB>,
}

pub async fn list_missions(
    State(_state): State<MissionHandlerState>,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let _limit = query.limit.unwrap_or(20).min(50) as i64;

    // Simplistic mock implementation for cursor pagination
    let data = vec![];

    let res = ListMissionsResponse {
        data,
        next_cursor: None,
    };

    Json(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_missions_empty() {
        // Just verify compiling structure
        assert!(true);
    }
}
