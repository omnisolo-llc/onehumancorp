use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::domain::repository::models::{FxRate, FxMargin, I18nString, LocalizedTransaction};
use crate::domain::repository::ledger_repo::LedgerRepository;

#[derive(Deserialize)]
pub struct SyncOfflineTransactionsRequest {
    pub tenant_id: String,
    pub transactions: Vec<LocalizedTransactionSyncPayload>,
}

#[derive(Deserialize)]
pub struct LocalizedTransactionSyncPayload {
    pub id: String,
    pub original_amount: f64,
    pub original_currency: String,
    pub target_currency: String,
    pub applied_fx_rate: f64,
    pub applied_margin: f64,
    pub final_amount: f64,
    pub created_at: chrono::DateTime<Utc>,
}

pub fn router() -> Router<crate::AppState> {
    Router::new()
        .route("/api/localization/fx-rates/:tenant_id", get(get_fx_rates))
        .route("/api/localization/fx-margins/:tenant_id", get(get_fx_margins))
        .route("/api/localization/i18n/:tenant_id/:language", get(get_i18n_strings))
        .route("/api/localization/sync", post(sync_offline_transactions))
}

async fn get_fx_rates(
    State(_state): State<crate::AppState>,
    Path(_tenant_id): Path<String>,
) -> impl IntoResponse {
    let rates = vec![
        FxRate {
            base_currency: "USD".to_string(),
            target_currency: "EUR".to_string(),
            rate: 0.92,
            updated_at: Utc::now(),
        },
        FxRate {
            base_currency: "USD".to_string(),
            target_currency: "GBP".to_string(),
            rate: 0.79,
            updated_at: Utc::now(),
        },
    ];
    (StatusCode::OK, Json(rates)).into_response()
}

async fn get_fx_margins(
    State(_state): State<crate::AppState>,
    Path(_tenant_id): Path<String>,
) -> impl IntoResponse {
    let margins = vec![
        FxMargin {
            base_currency: "USD".to_string(),
            target_currency: "EUR".to_string(),
            safe_margin: 0.03, // 3% buffer
        },
        FxMargin {
            base_currency: "USD".to_string(),
            target_currency: "GBP".to_string(),
            safe_margin: 0.03,
        },
    ];
    (StatusCode::OK, Json(margins)).into_response()
}

async fn get_i18n_strings(
    State(_state): State<crate::AppState>,
    Path((_tenant_id, language)): Path<(String, String)>,
) -> impl IntoResponse {
    let strings = if language == "es" {
        vec![
            I18nString { key: "greeting".to_string(), language: "es".to_string(), value: "Hola".to_string() },
            I18nString { key: "checkout".to_string(), language: "es".to_string(), value: "Pagar".to_string() },
            I18nString { key: "offline".to_string(), language: "es".to_string(), value: "Desconectado".to_string() },
        ]
    } else {
        vec![
            I18nString { key: "greeting".to_string(), language: "en".to_string(), value: "Hello".to_string() },
            I18nString { key: "checkout".to_string(), language: "en".to_string(), value: "Checkout".to_string() },
            I18nString { key: "offline".to_string(), language: "en".to_string(), value: "Offline".to_string() },
        ]
    };
    (StatusCode::OK, Json(strings)).into_response()
}

async fn sync_offline_transactions(
    State(state): State<crate::AppState>,
    Json(payload): Json<SyncOfflineTransactionsRequest>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);

    for tx_payload in payload.transactions {
        let tx = LocalizedTransaction {
            id: tx_payload.id,
            tenant_id: payload.tenant_id.clone(),
            original_amount: tx_payload.original_amount,
            original_currency: tx_payload.original_currency,
            target_currency: tx_payload.target_currency,
            applied_fx_rate: tx_payload.applied_fx_rate,
            applied_margin: tx_payload.applied_margin,
            final_amount: tx_payload.final_amount,
            is_offline: true,
            reconciled: true,
            created_at: tx_payload.created_at,
        };

        if let Err(e) = repo.save_localized_transaction(tx).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response();
        }
    }

    StatusCode::OK.into_response()
}
