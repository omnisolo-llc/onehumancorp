use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use ohc_core::jobs::{Job, JobContext, JobResult};
use crate::domain::repository::models::{Product, Supplier, AgentActionRequest};
use ohc_core::llm::{LLMProvider, LLMMessage};
use serde_json::json;

pub struct RestockerJob {}

impl Job for RestockerJob {
    fn name(&self) -> &'static str {
        "restocker_job"
    }

    async fn execute(&self, ctx: JobContext) -> JobResult {
        let pool = ctx.db.clone();
        let tenant_id = ctx.tenant_id.clone();

        // 1. Find products low on stock
        let low_stock_products: Vec<(Product, Supplier)> = sqlx::query_as!(
            (Product, Supplier),
            r#"
            SELECT
                p.id as "p.id", p.tenant_id as "p.tenant_id", p.type as "p.type", p.title as "p.title",
                p.name as "p.name", p.description as "p.description", p.price as "p.price",
                p.price_cents as "p.price_cents", p.currency as "p.currency", p.in_stock as "p.in_stock",
                p.inventory_count as "p.inventory_count", p.locked_quantity as "p.locked_quantity",
                p.available_quantity as "p.available_quantity", p.is_sold_out as "p.is_sold_out",
                p.is_subscribable as "p.is_subscribable", p.subscription_frequency as "p.subscription_frequency",
                p.subscription_discount_percent as "p.subscription_discount_percent", p.metadata as "p.metadata",
                p.seo_title as "p.seo_title", p.seo_description as "p.seo_description",
                p.seo_schema_json as "p.seo_schema_json", p.created_at as "p.created_at",
                p.updated_at as "p.updated_at", p.low_stock_threshold as "p.low_stock_threshold",
                p.supplier_id as "p.supplier_id",
                s.id as "s.id", s.tenant_id as "s.tenant_id", s.name as "s.name", s.email as "s.email",
                s.phone as "s.phone", s.created_at as "s.created_at", s.updated_at as "s.updated_at"
            FROM products p
            JOIN suppliers s ON p.supplier_id = s.id
            WHERE p.tenant_id = $1 AND p.inventory_count <= p.low_stock_threshold
            "#,
            tenant_id
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query products: {}", e))?;

        for (product, supplier) in low_stock_products {
            // Check if there is already a pending request for this product
            let pending_count: i64 = sqlx::query_scalar!(
                "SELECT count(*) FROM agent_action_requests WHERE tenant_id = $1 AND product_id = $2 AND status = 'Pending' AND action_type = 'Reorder'",
                tenant_id, product.id
            )
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

            if pending_count > 0 {
                continue;
            }

            // Calculate suggested reorder quantity (e.g., 3 * threshold)
            let suggested_qty = product.low_stock_threshold.unwrap_or(10) * 3;

            // Use LLM to draft email
            let llm = ohc_core::llm::create_provider();
            let prompt = format!(
                "You are an AI assistant for a business. Draft a polite email to supplier '{}' (email: {}) to order {} units of product '{}'. Keep it brief and professional.",
                supplier.name, supplier.email.clone().unwrap_or_default(), suggested_qty, product.title.clone().unwrap_or_default()
            );

            let draft = llm.generate_text(vec![LLMMessage::user(prompt)]).await.unwrap_or_else(|_| "Please reorder stock.".to_string());

            let request_id = Uuid::new_v4().to_string();
            let payload = json!({
                "suggested_quantity": suggested_qty,
                "draft_email": draft,
                "supplier_name": supplier.name,
                "supplier_email": supplier.email,
                "product_name": product.title,
                "current_stock": product.inventory_count
            });

            sqlx::query!(
                "INSERT INTO agent_action_requests (id, tenant_id, action_type, status, product_id, payload) VALUES ($1, $2, 'Reorder', 'Pending', $3, $4)",
                request_id, tenant_id, product.id, payload
            )
            .execute(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to insert request: {}", e))?;
        }

        Ok(())
    }
}
