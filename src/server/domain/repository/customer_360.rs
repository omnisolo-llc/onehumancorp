use serde::Serialize;
use sqlx::Row;

#[derive(Serialize)]
pub struct Customer360Response {
    pub customer: serde_json::Value,
    pub orders: Vec<serde_json::Value>,
    pub bookings: Vec<serde_json::Value>,
    pub conversations: Vec<serde_json::Value>,
}


pub struct Customer360Repository;

impl Customer360Repository {
    pub async fn get_customer_360(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: &str,
        customer_id: &str,
    ) -> Result<Option<Customer360Response>, sqlx::Error> {
        let query = r#"
            SELECT json_build_object(
                'customer', row_to_json(c),
                'orders', COALESCE((SELECT json_agg(row_to_json(o)) FROM (SELECT * FROM orders WHERE customer_id = c.id AND tenant_id = c.tenant_id ORDER BY created_at DESC) o), '[]'::json),
                'bookings', COALESCE((SELECT json_agg(row_to_json(b)) FROM (SELECT * FROM bookings WHERE customer_id = c.id AND tenant_id = c.tenant_id ORDER BY start_time DESC) b), '[]'::json),
                'conversations', COALESCE((
                    SELECT json_agg(convs) FROM (
                        SELECT id as conversation_id, channel, content, created_at FROM interactions WHERE customer_id = c.id AND tenant_id = c.tenant_id
                        UNION ALL
                        SELECT conversation_id, channel, content, created_at FROM conversations WHERE customer_id = c.id AND tenant_id = c.tenant_id
                        ORDER BY created_at DESC
                    ) convs
                ), '[]'::json)
            ) as data
            FROM customers c
            WHERE c.id = $1 AND c.tenant_id = $2
        "#;

        let row = sqlx::query(query)
            .bind(customer_id)
            .bind(tenant_id)
            .fetch_optional(&mut **tx)
            .await?;

        let row = match row {
            Some(r) => r,
            None => return Ok(None),
        };

        let data: serde_json::Value = row.try_get("data")?;

        let customer = data.get("customer").cloned().unwrap_or(serde_json::json!({}));
        let orders = data.get("orders").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let bookings = data.get("bookings").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let conversations = data.get("conversations").and_then(|v| v.as_array()).cloned().unwrap_or_default();

        Ok(Some(Customer360Response {
            customer,
            orders,
            bookings,
            conversations,
        }))
    }
}
