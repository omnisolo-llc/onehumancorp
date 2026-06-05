use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use sqlx::Row;
use axum::http;
// Instead of crate prefix, we reference the module locally or just through the integration.
use ::server_integrations_pkpass::generator::{PkpassGenerator, PkpassData};

pub async fn download_pass(
    State(hub): State<Arc<crate::hub::Hub>>,
    Path(token): Path<String>,
) -> Result<Response, StatusCode> {
    // Lookup the pass by token
    let row_result: Result<sqlx::postgres::PgRow, sqlx::Error> = sqlx::query("SELECT tenant_id, customer_id, pass_type, payload FROM wallet_passes WHERE token = $1")
        .bind(&token)
        .fetch_one(&hub.pool)
        .await;

    let row = match row_result {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let tenant_id: String = row.get("tenant_id");
    let customer_id: String = row.get("customer_id");
    let pass_type: String = row.get("pass_type");
    let _payload: serde_json::Value = row.get("payload");

    // Fetch tenant configuration for branding
    let tenant_row_result: Result<sqlx::postgres::PgRow, sqlx::Error> = sqlx::query("SELECT name FROM tenants WHERE id = $1")
        .bind(&tenant_id)
        .fetch_one(&hub.pool)
        .await;

    let organization_name = match tenant_row_result {
        Ok(tr) => tr.get("name"),
        Err(_) => "OHC Merchant".to_string(),
    };

    let generator = PkpassGenerator::new();
    let data = PkpassData {
        pass_type_identifier: format!("pass.store.ohc.{}", tenant_id),
        team_identifier: "ABCDEFGHIJ".to_string(), // Dummy team ID for Apple
        serial_number: customer_id.clone(),
        organization_name,
        description: format!("Wallet pass for {}", pass_type),
        foreground_color: None,
        background_color: None,
    };

    let zip_bytes = match generator.generate(data) {
        Ok(bytes) => bytes,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/vnd.apple.pkpass")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"pass.pkpass\""))
        .body(axum::body::Body::from(zip_bytes))
        .unwrap();

    Ok(response)
}
