use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::proto::{FulfillmentOrder, Courier, Location, FulfillmentState, FulfillmentMethod};

#[derive(FromRow)]
pub struct FulfillmentOrderRow {
    pub id: Uuid,
    pub tenant_id: String,
    pub order_id: String,
    pub assigned_method: i32,
    pub state: i32,
    pub courier_id: Option<String>,
    pub origin_lat: Option<f64>,
    pub origin_lon: Option<f64>,
    pub origin_addr: Option<String>,
    pub dest_lat: Option<f64>,
    pub dest_lon: Option<f64>,
    pub dest_addr: Option<String>,
    pub estimated_prep_time_ms: i64,
    pub estimated_delivery_time_ms: i64,
    pub estimated_cost: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct CourierRow {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub method: i32,
    pub contact_info: String,
    pub cost_per_mile: f64,
    pub base_cost: f64,
    pub is_available: bool,
}

pub struct FulfillmentDb {
    pool: PgPool,
}

impl FulfillmentDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_order(&self, order: &FulfillmentOrder) -> Result<(), sqlx::Error> {
        let id = Uuid::parse_str(&order.id).unwrap_or_else(|_| Uuid::new_v4());

        sqlx::query!(
            r#"
            INSERT INTO fulfillment_orders (
                id, tenant_id, order_id, assigned_method, state, courier_id,
                origin_lat, origin_lon, origin_addr, dest_lat, dest_lon, dest_addr,
                estimated_prep_time_ms, estimated_delivery_time_ms, estimated_cost,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                to_timestamp($16), to_timestamp($17)
            )
            "#,
            id,
            order.tenant_id,
            order.order_id,
            order.assigned_method,
            order.state,
            if order.courier_id.is_empty() { None } else { Some(&order.courier_id) },
            order.origin.as_ref().map(|l| l.latitude),
            order.origin.as_ref().map(|l| l.longitude),
            order.origin.as_ref().map(|l| l.address.clone()),
            order.destination.as_ref().map(|l| l.latitude),
            order.destination.as_ref().map(|l| l.longitude),
            order.destination.as_ref().map(|l| l.address.clone()),
            order.estimated_prep_time_ms,
            order.estimated_delivery_time_ms,
            order.estimated_cost,
            (order.created_at_unix / 1000) as f64,
            (order.updated_at_unix / 1000) as f64,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_order_state(&self, tenant_id: &str, fulfillment_id: &str, state: i32) -> Result<(), sqlx::Error> {
        let id = Uuid::parse_str(fulfillment_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        sqlx::query!(
            r#"
            UPDATE fulfillment_orders
            SET state = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            "#,
            state,
            id,
            tenant_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_order(&self, tenant_id: &str, fulfillment_id: &str) -> Result<Option<FulfillmentOrder>, sqlx::Error> {
        let id = Uuid::parse_str(fulfillment_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let row = sqlx::query_as!(
            FulfillmentOrderRow,
            r#"
            SELECT * FROM fulfillment_orders
            WHERE id = $1 AND tenant_id = $2
            "#,
            id,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| FulfillmentOrder {
            id: r.id.to_string(),
            tenant_id: r.tenant_id,
            order_id: r.order_id,
            assigned_method: r.assigned_method,
            state: r.state,
            courier_id: r.courier_id.unwrap_or_default(),
            origin: Some(Location {
                latitude: r.origin_lat.unwrap_or_default(),
                longitude: r.origin_lon.unwrap_or_default(),
                address: r.origin_addr.unwrap_or_default(),
            }),
            destination: Some(Location {
                latitude: r.dest_lat.unwrap_or_default(),
                longitude: r.dest_lon.unwrap_or_default(),
                address: r.dest_addr.unwrap_or_default(),
            }),
            estimated_prep_time_ms: r.estimated_prep_time_ms,
            estimated_delivery_time_ms: r.estimated_delivery_time_ms,
            estimated_cost: r.estimated_cost,
            created_at_unix: r.created_at.timestamp_millis(),
            updated_at_unix: r.updated_at.timestamp_millis(),
        }))
    }

    pub async fn get_available_couriers(&self, tenant_id: &str) -> Result<Vec<Courier>, sqlx::Error> {
        let rows = sqlx::query_as!(
            CourierRow,
            r#"
            SELECT * FROM couriers
            WHERE tenant_id = $1 AND is_available = true
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Courier {
            id: r.id.to_string(),
            name: r.name,
            method: r.method,
            contact_info: r.contact_info,
            cost_per_mile: r.cost_per_mile,
            base_cost: r.base_cost,
            is_available: r.is_available,
        }).collect())
    }

    pub async fn register_courier(&self, courier: &Courier, tenant_id: &str) -> Result<(), sqlx::Error> {
        let id = Uuid::parse_str(&courier.id).unwrap_or_else(|_| Uuid::new_v4());

        sqlx::query!(
            r#"
            INSERT INTO couriers (
                id, tenant_id, name, method, contact_info, cost_per_mile, base_cost, is_available
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
            id,
            tenant_id,
            courier.name,
            courier.method,
            courier.contact_info,
            courier.cost_per_mile,
            courier.base_cost,
            courier.is_available,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
