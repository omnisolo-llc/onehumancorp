use sqlx::{PgPool, Row};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestoneData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub reached: bool,
}

pub struct MilestonesService {
    pool: PgPool,
}

impl MilestonesService {
    pub fn new(pool: PgPool) -> Self {
        MilestonesService { pool }
    }

    pub async fn track_visitor(&self, org_id: &str, visitor_id: &str, page_url: &str, referrer: Option<&str>) -> Result<bool, String> {
        let id = format!("vis-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let created_at = Utc::now().timestamp();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO storefront_visitors (id, organization_id, visitor_id, page_url, referrer, created_at_unix) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&id)
            .bind(org_id)
            .bind(visitor_id)
            .bind(page_url)
            .bind(referrer)
            .bind(created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let _ = self.evaluate_visitor_milestone(org_id).await;

        Ok(true)
    }

    async fn evaluate_visitor_milestone(&self, org_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT count(*) FROM storefront_visitors WHERE organization_id = $1")
            .bind(org_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);

        if count >= 100 {
            let exists = sqlx::query("SELECT id FROM business_milestones WHERE organization_id = $1 AND milestone_type = '100_visitors'")
                .bind(org_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            if exists.is_none() {
                let id = format!("ms-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
                let created_at = Utc::now().timestamp();
                sqlx::query("INSERT INTO business_milestones (id, organization_id, milestone_type, title, description, reached, reached_at, created_at_unix) VALUES ($1, $2, '100_visitors', '🚀 100 Visitors Today!', 'You reached 100 visitors!', true, CURRENT_TIMESTAMP, $3)")
                    .bind(&id)
                    .bind(org_id)
                    .bind(created_at)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn evaluate_order_milestone(&self, org_id: &str, order_count: i32) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let mut trigger_milestone = None;
        if order_count >= 10 {
            trigger_milestone = Some(("10th_order", "🎉 10th Order!", "You completed 10 orders!"));
        } else if order_count >= 3 {
            trigger_milestone = Some(("3rd_order", "🎉 3rd Order!", "You completed 3 orders!"));
        } else if order_count >= 1 {
            trigger_milestone = Some(("1st_order", "First Sale!", "You got your first order!"));
        }

        if let Some((m_type, title, desc)) = trigger_milestone {
            let exists = sqlx::query("SELECT id FROM business_milestones WHERE organization_id = $1 AND milestone_type = $2")
                .bind(org_id)
                .bind(m_type)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            if exists.is_none() {
                let id = format!("ms-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
                let created_at = Utc::now().timestamp();
                sqlx::query("INSERT INTO business_milestones (id, organization_id, milestone_type, title, description, reached, reached_at, created_at_unix) VALUES ($1, $2, $3, $4, $5, true, CURRENT_TIMESTAMP, $6)")
                    .bind(&id)
                    .bind(org_id)
                    .bind(m_type)
                    .bind(title)
                    .bind(desc)
                    .bind(created_at)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_milestones(&self, org_id: &str) -> Result<Vec<MilestoneData>, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT id, title, description, reached FROM business_milestones WHERE organization_id = $1 ORDER BY created_at_unix DESC")
            .bind(org_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let milestones = rows.into_iter().map(|row| MilestoneData {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            reached: row.get("reached"),
        }).collect();

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(milestones)
    }

    pub async fn evaluate_free_tier(&self, org_id: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT plan_tier FROM organizations WHERE id = $1")
            .bind(org_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let plan: String = row.get("plan_tier");

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(plan.to_lowercase() == "free")
    }
}
