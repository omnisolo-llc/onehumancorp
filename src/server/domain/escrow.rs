use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectEscrow {
    pub id: String,
    pub tenant_id: String,
    pub total_amount: f64,
    pub fbo_account_id: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EscrowMilestone {
    pub id: String,
    pub escrow_id: String,
    pub tenant_id: String,
    pub release_amount: f64,
    pub status: String,
    pub proof_required: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LedgerTransaction {
    pub id: String,
    pub escrow_id: String,
    pub tenant_id: String,
    pub amount: f64,
    pub from_account: String,
    pub to_account: String,
    pub timestamp: Option<DateTime<Utc>>,
}

impl DB {
    pub async fn create_escrow(&self, tenant_id: &str, total_amount: f64, fbo_account_id: &str) -> Result<ProjectEscrow, String> {
        let id = Uuid::new_v4().to_string();
        let status = "FUNDED".to_string();

        if self.is_sqlite() {
            if let crate::db::DbStore::Sqlite(pool) = &self.store {
                sqlx::query("INSERT INTO project_escrows (id, tenant_id, total_amount, fbo_account_id, status) VALUES (?, ?, ?, ?, ?) RETURNING id, tenant_id, total_amount, fbo_account_id, status, created_at, updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(total_amount)
                .bind(fbo_account_id)
                .bind(&status)
                .fetch_one(pool)
                .await
                .map(|r| ProjectEscrow {
                    id: r.get("id"),
                    tenant_id: r.get("tenant_id"),
                    total_amount: r.get("total_amount"),
                    fbo_account_id: r.get("fbo_account_id"),
                    status: r.get("status"),
                    created_at: r.try_get::<i64, _>("created_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                    updated_at: r.try_get::<i64, _>("updated_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                })
                .map_err(|e| e.to_string())
            } else {
                Err("Invalid DB store".to_string())
            }
        } else {
            sqlx::query("INSERT INTO project_escrows (id, tenant_id, total_amount, fbo_account_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, total_amount, fbo_account_id, status, created_at, updated_at")
            .bind(&id)
            .bind(tenant_id)
            .bind(total_amount)
            .bind(fbo_account_id)
            .bind(&status)
            .fetch_one(&self.pool)
            .await
            .map(|r| ProjectEscrow {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                total_amount: r.get("total_amount"),
                fbo_account_id: r.get("fbo_account_id"),
                status: r.get("status"),
                created_at: r.try_get("created_at").ok(),
                updated_at: r.try_get("updated_at").ok(),
            })
            .map_err(|e| e.to_string())
        }
    }

    pub async fn add_milestone(&self, escrow_id: &str, tenant_id: &str, release_amount: f64, proof_required: &str) -> Result<EscrowMilestone, String> {
        let id = Uuid::new_v4().to_string();
        let status = "PENDING".to_string();

        if self.is_sqlite() {
            if let crate::db::DbStore::Sqlite(pool) = &self.store {
                sqlx::query("INSERT INTO escrow_milestones (id, escrow_id, tenant_id, release_amount, status, proof_required) VALUES (?, ?, ?, ?, ?, ?) RETURNING id, escrow_id, tenant_id, release_amount, status, proof_required, created_at, updated_at")
                .bind(&id)
                .bind(escrow_id)
                .bind(tenant_id)
                .bind(release_amount)
                .bind(&status)
                .bind(proof_required)
                .fetch_one(pool)
                .await
                .map(|r| EscrowMilestone {
                    id: r.get("id"),
                    escrow_id: r.get("escrow_id"),
                    tenant_id: r.get("tenant_id"),
                    release_amount: r.get("release_amount"),
                    status: r.get("status"),
                    proof_required: r.get("proof_required"),
                    created_at: r.try_get::<i64, _>("created_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                    updated_at: r.try_get::<i64, _>("updated_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                })
                .map_err(|e| e.to_string())
            } else {
                Err("Invalid DB store".to_string())
            }
        } else {
            sqlx::query("INSERT INTO escrow_milestones (id, escrow_id, tenant_id, release_amount, status, proof_required) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, escrow_id, tenant_id, release_amount, status, proof_required, created_at, updated_at")
            .bind(&id)
            .bind(escrow_id)
            .bind(tenant_id)
            .bind(release_amount)
            .bind(&status)
            .bind(proof_required)
            .fetch_one(&self.pool)
            .await
            .map(|r| EscrowMilestone {
                id: r.get("id"),
                escrow_id: r.get("escrow_id"),
                tenant_id: r.get("tenant_id"),
                release_amount: r.get("release_amount"),
                status: r.get("status"),
                proof_required: r.get("proof_required"),
                created_at: r.try_get("created_at").ok(),
                updated_at: r.try_get("updated_at").ok(),
            })
            .map_err(|e| e.to_string())
        }
    }

    pub async fn get_milestone(&self, milestone_id: &str, tenant_id: &str) -> Result<EscrowMilestone, String> {
        if self.is_sqlite() {
            if let crate::db::DbStore::Sqlite(pool) = &self.store {
                sqlx::query("SELECT id, escrow_id, tenant_id, release_amount, status, proof_required, created_at, updated_at FROM escrow_milestones WHERE id = ? AND tenant_id = ?")
                .bind(milestone_id)
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map(|r| EscrowMilestone {
                    id: r.get("id"),
                    escrow_id: r.get("escrow_id"),
                    tenant_id: r.get("tenant_id"),
                    release_amount: r.get("release_amount"),
                    status: r.get("status"),
                    proof_required: r.get("proof_required"),
                    created_at: r.try_get::<i64, _>("created_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                    updated_at: r.try_get::<i64, _>("updated_at").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                })
                .map_err(|e| e.to_string())
            } else {
                Err("Invalid DB store".to_string())
            }
        } else {
            sqlx::query("SELECT id, escrow_id, tenant_id, release_amount, status, proof_required, created_at, updated_at FROM escrow_milestones WHERE id = $1 AND tenant_id = $2")
            .bind(milestone_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .map(|r| EscrowMilestone {
                id: r.get("id"),
                escrow_id: r.get("escrow_id"),
                tenant_id: r.get("tenant_id"),
                release_amount: r.get("release_amount"),
                status: r.get("status"),
                proof_required: r.get("proof_required"),
                created_at: r.try_get("created_at").ok(),
                updated_at: r.try_get("updated_at").ok(),
            })
            .map_err(|e| e.to_string())
        }
    }

    pub async fn approve_milestone(&self, milestone_id: &str, tenant_id: &str) -> Result<LedgerTransaction, String> {
        let milestone = self.get_milestone(milestone_id, tenant_id).await?;

        if milestone.status == "APPROVED" {
            return Err("Milestone already approved".to_string());
        }

        let transaction_id = Uuid::new_v4().to_string();

        if self.is_sqlite() {
            if let crate::db::DbStore::Sqlite(pool) = &self.store {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query("UPDATE escrow_milestones SET status = 'APPROVED' WHERE id = ? AND tenant_id = ?")
                .bind(milestone_id)
                .bind(tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = sqlx::query("INSERT INTO ledger_transactions (id, escrow_id, tenant_id, amount, from_account, to_account) VALUES (?, ?, ?, ?, 'escrow', 'owner_wallet') RETURNING id, escrow_id, tenant_id, amount, from_account, to_account, timestamp")
                .bind(&transaction_id)
                .bind(&milestone.escrow_id)
                .bind(tenant_id)
                .bind(milestone.release_amount)
                .fetch_one(&mut *tx)
                .await
                .map(|r| LedgerTransaction {
                    id: r.get("id"),
                    escrow_id: r.get("escrow_id"),
                    tenant_id: r.get("tenant_id"),
                    amount: r.get("amount"),
                    from_account: r.get("from_account"),
                    to_account: r.get("to_account"),
                    timestamp: r.try_get::<i64, _>("timestamp").ok().map(|dt| chrono::DateTime::from_timestamp(dt, 0).unwrap()),
                })
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(res)
            } else {
                Err("Invalid DB store".to_string())
            }
        } else {
            let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

            sqlx::query("UPDATE escrow_milestones SET status = 'APPROVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(milestone_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            let res = sqlx::query("INSERT INTO ledger_transactions (id, escrow_id, tenant_id, amount, from_account, to_account) VALUES ($1, $2, $3, $4, 'escrow', 'owner_wallet') RETURNING id, escrow_id, tenant_id, amount, from_account, to_account, timestamp")
            .bind(&transaction_id)
            .bind(&milestone.escrow_id)
            .bind(tenant_id)
            .bind(milestone.release_amount)
            .fetch_one(&mut *tx)
            .await
            .map(|r| LedgerTransaction {
                id: r.get("id"),
                escrow_id: r.get("escrow_id"),
                tenant_id: r.get("tenant_id"),
                amount: r.get("amount"),
                from_account: r.get("from_account"),
                to_account: r.get("to_account"),
                timestamp: r.try_get("timestamp").ok(),
            })
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;

    #[tokio::test]
    async fn test_create_and_approve_escrow() {
        if std::env::var("DATABASE_URL").is_err() { return; }

        let db = DB::new().await;

        // 1. Create Escrow
        let escrow = db.create_escrow("tenant_test", 1500.0, "fbo_123").await.expect("Failed to create escrow");
        assert_eq!(escrow.status, "FUNDED");
        assert_eq!(escrow.total_amount, 1500.0);

        // 2. Add Milestone
        let milestone = db.add_milestone(&escrow.id, "tenant_test", 500.0, "Photo of framing").await.expect("Failed to add milestone");
        assert_eq!(milestone.status, "PENDING");
        assert_eq!(milestone.release_amount, 500.0);

        // 3. Approve Milestone
        let tx = db.approve_milestone(&milestone.id, "tenant_test").await.expect("Failed to approve milestone");
        assert_eq!(tx.amount, 500.0);
        assert_eq!(tx.from_account, "escrow");
        assert_eq!(tx.to_account, "owner_wallet");

        // 4. Verify Milestone Status Updated
        let updated_milestone = db.get_milestone(&milestone.id, "tenant_test").await.expect("Failed to get milestone");
        assert_eq!(updated_milestone.status, "APPROVED");
    }
}
