pub mod builder;
pub mod signer;
pub mod worker;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPass {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub pass_type: String, // "loyalty", "booking"
    pub status: String,    // "active", "expired", "revoked"
    pub pass_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassField {
    pub key: String,
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassBarcode {
    pub format: String, // e.g., "PKBarcodeFormatQR"
    pub message: String,
    pub message_encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleWalletPass {
    pub format_version: i32,
    pub pass_type_identifier: String,
    pub serial_number: String,
    pub team_identifier: String,
    pub organization_name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,

    // Pass Type Dictionaries (we use generic struct for simplicity, picking one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic: Option<PassStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_card: Option<PassStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticket: Option<PassStructure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcodes: Option<Vec<PassBarcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<PassBarcode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassStructure {
    #[serde(default)]
    pub primary_fields: Vec<PassField>,
    #[serde(default)]
    pub secondary_fields: Vec<PassField>,
    #[serde(default)]
    pub auxiliary_fields: Vec<PassField>,
    #[serde(default)]
    pub back_fields: Vec<PassField>,
}
