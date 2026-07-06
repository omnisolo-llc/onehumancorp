use uuid::Uuid;
use sqlx::{PgPool, Row};
use tracing::info;

pub struct ProcurementService {
    pool: PgPool,
}

impl ProcurementService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculates burn rate of a raw material based on finished good sales
    pub async fn calculate_burn_rate(&self, tenant_id: Uuid, raw_material_id: Uuid) -> Result<f64, sqlx::Error> {
        // Dummy implementation for burn rate
        info!("Calculating burn rate for raw material {} for tenant {}", raw_material_id, tenant_id);
        Ok(5.0) // Returns units consumed per day
    }

    /// Drafts a purchase order for a raw material
    pub async fn draft_purchase_order(&self, tenant_id: Uuid, raw_material_id: Uuid, quantity: f64) -> Result<Uuid, sqlx::Error> {
        // Fetch local supplier from directory
        let supplier_row = sqlx::query(
            r#"SELECT id FROM supplier_directory WHERE tenant_id = $1 LIMIT 1"#
        )
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        let supplier_id: Uuid = if let Some(row) = supplier_row {
            row.try_get("id")?
        } else {
            // No supplier found
            return Err(sqlx::Error::RowNotFound);
        };

        // Create PO
        let po_id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO purchase_orders (id, tenant_id, supplier_id, total_amount, status)
               VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(po_id)
        .bind(tenant_id)
        .bind(supplier_id)
        .bind(quantity * 10.0) // dummy amount calc
        .bind("pending_approval")
        .execute(&self.pool)
        .await?;

        info!("Drafted purchase order {} for tenant {}", po_id, tenant_id);
        Ok(po_id)
    }

    /// Approves a drafted purchase order
    pub async fn approve_purchase_order(&self, tenant_id: Uuid, po_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE purchase_orders SET status = 'approved' WHERE id = $1 AND tenant_id = $2"#
        )
        .bind(po_id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        info!("Approved purchase order {} for tenant {}", po_id, tenant_id);
        Ok(())
    }
}
