use crate::domain::repository::agent_feed_repo::{AgentFeedItem, AgentFeedRepository};
use sqlx::PgPool;
use tracing::info;
use serde_json::Value;

pub struct WorkTriageService {
    pool: PgPool,
}

impl WorkTriageService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest(&self, mut item: AgentFeedItem) -> Result<AgentFeedItem, sqlx::Error> {
        // 1. Calculate grouping (correlation_id) based on rules
        let correlation_id = self.calculate_correlation_id(&item);
        item.correlation_id = Some(correlation_id.clone());

        // 2. Calculate priority_score
        item.priority_score = Some(self.calculate_priority_score(&item));

        let repo = AgentFeedRepository::new(self.pool.clone());

        // 3. Check for existing pending item to deduplicate
        if let Ok(Some(mut existing)) = repo.get_pending_by_correlation_id(&item.tenant_id, &correlation_id).await {
            info!("Deduplicating triage item with correlation_id: {}", correlation_id);

            // Merge logic based on event source
            if existing.event_source == "low_stock" || existing.event_source == "inventory_alert" {
                self.merge_stock_alerts(&mut existing, &item);
                return repo.update(existing).await;
            } else if existing.event_source == "omnichannel_gateway" || existing.event_source == "message_triage" {
                self.merge_messages(&mut existing, &item);
                return repo.update(existing).await;
            }
        }

        // 4. Create new if not deduplicated
        repo.create(item).await
    }

    fn calculate_correlation_id(&self, item: &AgentFeedItem) -> String {
        // Simple rules for grouping
        if item.event_source == "low_stock" || item.event_source == "inventory_alert" {
            // Group by product id if available, or just general "low_stock"
            if let Some(payload) = &item.context_payload {
                if let Some(product_id) = payload.get("product_id").and_then(|v| v.as_str()) {
                    return format!("low_stock_{}", product_id);
                }
            }
            return "low_stock_general".to_string();
        } else if item.event_source == "omnichannel_gateway" || item.event_source == "message_triage" {
            if let Some(payload) = &item.context_payload {
                if let Some(customer_id) = payload.get("customer_id").and_then(|v| v.as_str()) {
                    return format!("message_customer_{}", customer_id);
                }
            }
            return "message_general".to_string();
        } else if item.event_source == "payment_failed" || item.event_source == "deposit_failed" {
            if let Some(payload) = &item.context_payload {
                if let Some(order_id) = payload.get("order_id").and_then(|v| v.as_str()) {
                    return format!("payment_failed_{}", order_id);
                }
            }
            return "payment_failed_general".to_string();
        }

        // Fallback to item ID if no grouping rule
        item.id.clone()
    }

    fn calculate_priority_score(&self, item: &AgentFeedItem) -> i32 {
        let source = item.event_source.as_str();
        match source {
            "payment_failed" | "deposit_failed" => 100, // Urgent
            "booking_request" | "quote_request" => 80, // High revenue opportunity
            "low_stock" | "inventory_alert" => 50, // Medium ops issue
            "omnichannel_gateway" | "message_triage" => 30, // Normal message
            _ => 10, // Low priority
        }
    }

    fn merge_stock_alerts(&self, existing: &mut AgentFeedItem, new_item: &AgentFeedItem) {
        // Increment a counter in context_payload to represent grouped alerts
        let mut context = existing.context_payload.clone().map(|v| v.0).unwrap_or_else(|| serde_json::json!({}));
        let mut count = context.get("grouped_count").and_then(|v| v.as_i64()).unwrap_or(1);
        count += 1;
        context["grouped_count"] = serde_json::json!(count);

        // Update priority if new item is higher
        if let (Some(existing_p), Some(new_p)) = (existing.priority_score, new_item.priority_score) {
            if new_p > existing_p {
                existing.priority_score = Some(new_p);
            }
        }

        existing.context_payload = Some(sqlx::types::Json(context));
        existing.updated_at = Some(chrono::Utc::now());
    }

    fn merge_messages(&self, existing: &mut AgentFeedItem, new_item: &AgentFeedItem) {
        let mut context = existing.context_payload.clone().map(|v| v.0).unwrap_or_else(|| serde_json::json!({}));
        let mut count = context.get("grouped_count").and_then(|v| v.as_i64()).unwrap_or(1);
        count += 1;
        context["grouped_count"] = serde_json::json!(count);

        // Update priority if new item is higher
        if let (Some(existing_p), Some(new_p)) = (existing.priority_score, new_item.priority_score) {
            if new_p > existing_p {
                existing.priority_score = Some(new_p);
            }
        }

        existing.context_payload = Some(sqlx::types::Json(context));
        existing.updated_at = Some(chrono::Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_calculate_correlation_id() {
        // Setup mock environment if needed, but the logic is pure
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(); // Won't actually connect lazy
        let service = WorkTriageService::new(pool);

        let item1 = AgentFeedItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: "t1".to_string(),
            event_source: "low_stock".to_string(),
            context_payload: Some(sqlx::types::Json(serde_json::json!({"product_id": "p123"}))),
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            correlation_id: None,
            priority_score: None,
        };

        assert_eq!(service.calculate_correlation_id(&item1), "low_stock_p123");

        let item2 = AgentFeedItem {
            id: "id_123".to_string(),
            tenant_id: "t1".to_string(),
            event_source: "other".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            correlation_id: None,
            priority_score: None,
        };

        assert_eq!(service.calculate_correlation_id(&item2), "id_123");
    }

    #[tokio::test]
    async fn test_calculate_priority_score() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = WorkTriageService::new(pool);

        let mut item = AgentFeedItem {
            id: "id".to_string(),
            tenant_id: "t1".to_string(),
            event_source: "deposit_failed".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            correlation_id: None,
            priority_score: None,
        };

        assert_eq!(service.calculate_priority_score(&item), 100);

        item.event_source = "low_stock".to_string();
        assert_eq!(service.calculate_priority_score(&item), 50);

        item.event_source = "other".to_string();
        assert_eq!(service.calculate_priority_score(&item), 10);
    }

    #[tokio::test]
    async fn test_merge_items() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = WorkTriageService::new(pool);

        let mut existing = AgentFeedItem {
            id: "id1".to_string(),
            tenant_id: "t1".to_string(),
            event_source: "low_stock".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            correlation_id: Some("low_stock_general".to_string()),
            priority_score: Some(50),
        };

        let new_item = AgentFeedItem {
            id: "id2".to_string(),
            tenant_id: "t1".to_string(),
            event_source: "low_stock".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            correlation_id: Some("low_stock_general".to_string()),
            priority_score: Some(55),
        };

        service.merge_stock_alerts(&mut existing, &new_item);

        assert_eq!(existing.priority_score, Some(55));
        let ctx = existing.context_payload.unwrap().0;
        assert_eq!(ctx.get("grouped_count").unwrap().as_i64().unwrap(), 2);
    }
}
