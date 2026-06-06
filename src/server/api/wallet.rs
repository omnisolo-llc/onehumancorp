use axum::{
    extract::{State, Extension},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::db::DB;
use crate::auth::AuthenticatedUser;
use crate::domain::repository::wallet_repo::WalletRepository;
use crate::domain::repository::models::{Wallet, VirtualCard};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/api/v1/wallet", get(get_wallet))
        .route("/api/v1/wallet/virtual-card", get(get_virtual_card))
        .route("/api/v1/wallet/virtual-card/reveal", post(reveal_virtual_card))
}

#[derive(Serialize)]
pub struct WalletResponse {
    pub wallet: Option<Wallet>,
}

#[derive(Serialize)]
pub struct VirtualCardResponse {
    pub card: Option<VirtualCard>,
}

#[derive(Serialize)]
pub struct RevealVirtualCardResponse {
    pub pan: String,
    pub cvc: String,
    pub expiry_month: i32,
    pub expiry_year: i32,
}

async fn get_wallet(
    State(db): State<Arc<DB>>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<WalletResponse>, axum::http::StatusCode> {
    let repo = WalletRepository::new(db);
    let tenant_id = user.organization_id;

    let wallet = match repo.get_wallet_by_tenant(&tenant_id).await {
        Ok(Some(w)) => w,
        Ok(None) => {
            // Auto-create wallet if it doesn't exist for the tenant
            let new_wallet = Wallet {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.clone(),
                available_balance_cents: 0,
                currency: "USD".to_string(),
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            };
            repo.create_wallet(new_wallet.clone()).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        },
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(WalletResponse { wallet: Some(wallet) }))
}

async fn get_virtual_card(
    State(db): State<Arc<DB>>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<VirtualCardResponse>, axum::http::StatusCode> {
    let repo = WalletRepository::new(db.clone());
    let tenant_id = user.organization_id;

    // Ensure wallet exists
    let wallet = match repo.get_wallet_by_tenant(&tenant_id).await {
        Ok(Some(w)) => w,
        Ok(None) => {
            let new_wallet = Wallet {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.clone(),
                available_balance_cents: 0,
                currency: "USD".to_string(),
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            };
            repo.create_wallet(new_wallet.clone()).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        },
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    let card = match repo.get_virtual_card_by_tenant(&tenant_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            // Auto-create virtual card if it doesn't exist
            let new_card = VirtualCard {
                id: Uuid::new_v4().to_string(),
                wallet_id: wallet.id,
                tenant_id: tenant_id.clone(),
                status: "ACTIVE".to_string(),
                tokenized_pan: "tok_123456789".to_string(),
                last_four: "4242".to_string(),
                expiry_month: 12,
                expiry_year: 2028,
                cardholder_name: "Business Owner".to_string(),
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            };
            repo.create_virtual_card(new_card.clone()).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        },
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(VirtualCardResponse { card: Some(card) }))
}

async fn reveal_virtual_card(
    State(db): State<Arc<DB>>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<RevealVirtualCardResponse>, axum::http::StatusCode> {
    let repo = WalletRepository::new(db);
    let tenant_id = user.organization_id;

    let card = repo.get_virtual_card_by_tenant(&tenant_id).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // In a real app, this would call a secure vault (e.g. Stripe Issuing, Marqeta) to get the true PAN and CVC
    Ok(Json(RevealVirtualCardResponse {
        pan: format!("4242 4242 4242 {}", card.last_four),
        cvc: "123".to_string(),
        expiry_month: card.expiry_month,
        expiry_year: card.expiry_year,
    }))
}
