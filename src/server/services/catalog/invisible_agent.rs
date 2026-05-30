use std::sync::Arc;

#[derive(Clone)]
pub struct InvisibleCatalogAgent {
    pub db: Arc<crate::db::DB>,
    pub hub: Arc<crate::hub::Hub>,
}

impl InvisibleCatalogAgent {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self { db, hub }
    }

    pub async fn process_video_scan(&self, tenant_id: &str, video_url: &str) -> Result<String, String> {
        let scan_id = uuid::Uuid::new_v4().to_string();

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO product_video_scans (id, tenant_id, video_url, status) VALUES ($1, $2, $3, 'PROCESSING')"
        )
        .bind(&scan_id)
        .bind(tenant_id)
        .bind(video_url)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Mock CV process extracting drafts
        let drafts = vec![
            ("Chocolate Croissant", "Delicious buttery pastry with chocolate", 450, "https://example.com/croissant.jpg"),
            ("Vanilla Cupcake", "Classic vanilla with buttercream frosting", 300, "https://example.com/cupcake.jpg"),
            ("Strawberry Tart", "Fresh strawberries on a sweet crust", 550, "https://example.com/tart.jpg"),
        ];

        for (name, desc, price, img) in drafts {
            let draft_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO draft_catalog_items (id, scan_id, tenant_id, name, description, estimated_price_cents, image_url, status)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING_REVIEW')"
            )
            .bind(&draft_id)
            .bind(&scan_id)
            .bind(tenant_id)
            .bind(name)
            .bind(desc)
            .bind(price)
            .bind(img)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        sqlx::query(
            "UPDATE product_video_scans SET status = 'COMPLETED' WHERE id = $1"
        )
        .bind(&scan_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(scan_id)
    }

    pub async fn review_draft_item(&self, tenant_id: &str, draft_id: &str, approved: bool) -> Result<Option<String>, String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let draft_row = sqlx::query(
            "SELECT id, name, description, estimated_price_cents, image_url FROM draft_catalog_items WHERE id = $1 AND tenant_id = $2"
        )
        .bind(draft_id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        use sqlx::Row;
        let image_url: Option<String> = draft_row.get("image_url");
        let name: String = draft_row.get("name");
        let description: Option<String> = draft_row.get("description");
        let estimated_price_cents: Option<i64> = draft_row.get("estimated_price_cents");

        let new_status = if approved { "APPROVED" } else { "REJECTED" };

        sqlx::query(
            "UPDATE draft_catalog_items SET status = $1 WHERE id = $2"
        )
        .bind(new_status)
        .bind(draft_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut product_id_opt = None;
        if approved {
            let product_id = uuid::Uuid::new_v4().to_string();
            let metadata = serde_json::json!({
                "image_url": image_url,
                "source": "invisible_catalog_agent"
            });

            sqlx::query(
                "INSERT INTO products (id, tenant_id, title, description, type, price_cents, currency, metadata)
                 VALUES ($1, $2, $3, $4, 'physical', $5, 'USD', $6)"
            )
            .bind(&product_id)
            .bind(tenant_id)
            .bind(&name)
            .bind(&description)
            .bind(estimated_price_cents.unwrap_or(0))
            .bind(metadata)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            product_id_opt = Some(product_id);
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(product_id_opt)
    }
}
