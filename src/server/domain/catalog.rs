use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_create_product(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    let title = payload.get("title").or_else(|| payload.get("name")).and_then(|v| v.as_str()).unwrap_or("New Product");
    let description = payload.get("description").and_then(|v| v.as_str()).unwrap_or("");

    let price_str = payload.get("price").and_then(|v| {
        if v.is_string() { v.as_str() }
        else if v.is_number() { None } // Will handle in a moment if it's a number
        else { None }
    });

    let price_f64 = if let Some(s) = price_str {
        s.parse::<f64>().unwrap_or(0.0)
    } else if let Some(n) = payload.get("price").and_then(|v| v.as_f64()) {
        n
    } else {
        0.0
    };

    let price_cents = (price_f64 * 100.0).round() as i64;
    let item_type = payload.get("item_type").and_then(|v| v.as_str()).unwrap_or("Product");
    let product_id = uuid::Uuid::new_v4().to_string();

    tracing::info!("Creating product via action_router for tenant: {}, title: {}", tenant_id, title); // pii-safe

    sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, price_cents, inventory_count, is_subscribable, subscription_frequency, subscription_discount_percent) VALUES ($1, $2, $3, $4, $5, $6, 100, false, null, 0)"
    )
    .bind(&product_id)
    .bind(tenant_id)
    .bind(title)
    .bind(description)
    .bind(item_type)
    .bind(price_cents)
    .execute(pool)
    .await?;

    Ok(())
}
