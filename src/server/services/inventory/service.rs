use uuid::Uuid;

pub struct InventoryService;

impl InventoryService {
    pub async fn update_inventory(tenant_id: &str, product_id: &str, variant_id: &str, change: i32, reason: &str, transaction_id: &str) -> Result<(), String> {
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let id = Uuid::new_v4().to_string();

        let existing: Option<(i32,)> = sqlx::query_as(
            "SELECT 1::INT FROM inventory_ledger WHERE transaction_id = $1 AND tenant_id = $2"
        )
        .bind(transaction_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if existing.is_some() {
            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(());
        }

        sqlx::query(
            "INSERT INTO inventory_ledger (id, tenant_id, catalog_item_id, variant_id, change_amount, reason, transaction_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(product_id)
        .bind(variant_id)
        .bind(change)
        .bind(reason)
        .bind(transaction_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE products SET inventory_count = inventory_count + $1 WHERE id = $2 AND tenant_id = $3"
        )
        .bind(change)
        .bind(product_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub async fn setup_inventory_ledger_table(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS inventory_ledger (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            catalog_item_id TEXT NOT NULL,
            variant_id TEXT NOT NULL,
            change_amount INT NOT NULL,
            reason TEXT NOT NULL,
            transaction_id TEXT NOT NULL,
            UNIQUE(tenant_id, transaction_id)
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_inventory_idempotency_compiles() {
        assert!(true);
    }
}
