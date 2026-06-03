use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub currency: String,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub qr_payload: String, // Cryptographically signed payload for offline verification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerTransaction {
    pub id: String,
    pub tenant_id: String,
    pub gift_card_id: String,
    pub amount: f64, // negative for redemption, positive for refund/credit
    pub transaction_type: TransactionType,
    pub timestamp: SystemTime,
    pub offline_synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Redemption,
    RefundToCredit,
    InitialIssuance,
}
