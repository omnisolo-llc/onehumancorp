use chrono::{DateTime, Utc};
use sqlx::Row;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeamInvite {
    pub id: String,
    pub team_id: String,
    pub inviter_id: String,
    pub invitee_id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct InviteRepository {
    pool: sqlx::PgPool,
}

impl InviteRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        InviteRepository { pool }
    }

    pub async fn create_invite(&self, invite: &TeamInvite) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &invite.team_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&invite.id)
        .bind(&invite.team_id)
        .bind(&invite.inviter_id)
        .bind(&invite.invitee_id)
        .bind(&invite.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_team_invites_count(&self, team_id: &str) -> Result<i64, String> {
        let row = sqlx::query("SELECT COUNT(*) FROM team_invites WHERE team_id = $1")
            .bind(team_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        let count: i64 = row.get(0);
        Ok(count)
    }

    pub async fn get_total_invites_count(&self) -> Result<i64, String> {
        let row = sqlx::query("SELECT COUNT(*) FROM team_invites")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        let count: i64 = row.get(0);
        Ok(count)
    }

    pub async fn create_invites(&self, invites: &[TeamInvite]) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        for invite in invites {
            sqlx::query(
                "INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at) \
                 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&invite.id)
            .bind(&invite.team_id)
            .bind(&invite.inviter_id)
            .bind(&invite.invitee_id)
            .bind(&invite.status)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct InviteTracker {
    repo: Arc<InviteRepository>,
}

impl InviteTracker {
    pub fn new(repo: Arc<InviteRepository>) -> Self {
        InviteTracker { repo }
    }

    pub async fn record_invite(&self, team_id: &str, inviter_id: &str, invitee_id: &str) -> Result<(), String> {
        let invite = TeamInvite {
            id: format!("inv-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            team_id: team_id.to_string(),
            inviter_id: inviter_id.to_string(),
            invitee_id: invitee_id.to_string(),
            status: "PENDING".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create_invite(&invite).await?;
        
        Ok(())
    }

    pub async fn get_team_invites_count(&self, team_id: &str) -> Result<i64, String> {
        self.repo.get_team_invites_count(team_id).await
    }

    pub async fn get_total_invites_count(&self) -> Result<i64, String> {
        self.repo.get_total_invites_count().await
    }

    pub async fn record_invites(&self, team_id: &str, inviter_id: &str, invitee_ids: &[String]) -> Result<(), String> {
        let mut invites = Vec::new();
        for invitee_id in invitee_ids {
            invites.push(TeamInvite {
                id: format!("inv-{}-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0), invitee_id),
                team_id: team_id.to_string(),
                inviter_id: inviter_id.to_string(),
                invitee_id: invitee_id.clone(),
                status: "PENDING".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }

        self.repo.create_invites(&invites).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_team_invite_serialization() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let invite = TeamInvite {
            id: "inv1".to_string(),
            team_id: "team1".to_string(),
            inviter_id: "user1".to_string(),
            invitee_id: "user2".to_string(),
            status: "PENDING".to_string(),
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_string(&invite).unwrap();
        let deserialized: TeamInvite = serde_json::from_str(&json).unwrap();

        assert_eq!(invite.id, deserialized.id);
        assert_eq!(invite.status, deserialized.status);
        assert_eq!(invite.created_at, deserialized.created_at);
    }
}
