use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "agent_type_enum", rename_all = "PascalCase")]
pub enum AgentType {
    Ambassador,
    Promoter,
    Advisor,
    Operations,
    Finance,
    Legal,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "card_type_enum", rename_all = "PascalCase")]
pub enum CardType {
    Actionable,
    Info,
    Alert,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "card_status_enum", rename_all = "PascalCase")]
pub enum CardStatus {
    Pending,
    Approved,
    Dismissed,
    Edited,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AgentFeedCard {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub agent_type: AgentType,
    pub card_type: CardType,
    pub title: String,
    pub description: String,
    pub proposed_action_payload: Option<sqlx::types::Json<serde_json::Value>>,
    pub status: CardStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct AgentFeedService {
    pool: PgPool,
}

impl AgentFeedService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_card(
        &self,
        tenant_id: Uuid,
        agent_type: AgentType,
        card_type: CardType,
        title: String,
        description: String,
        proposed_action_payload: Option<serde_json::Value>,
    ) -> Result<AgentFeedCard, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Set RLS context
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let payload = proposed_action_payload.map(sqlx::types::Json);

        let card = sqlx::query_as::<_, AgentFeedCard>(
            r#"
            INSERT INTO agent_feed_cards (tenant_id, agent_type, card_type, title, description, proposed_action_payload)
            VALUES ($1, $2::agent_type_enum, $3::card_type_enum, $4, $5, $6)
            RETURNING id, tenant_id, agent_type AS "agent_type!: AgentType", card_type AS "card_type!: CardType", title, description, proposed_action_payload as "proposed_action_payload: sqlx::types::Json<serde_json::Value>", status AS "status!: CardStatus", created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(agent_type)
        .bind(card_type)
        .bind(title)
        .bind(description)
        .bind(payload)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(card)
    }

    pub async fn list_pending_cards(&self, tenant_id: Uuid) -> Result<Vec<AgentFeedCard>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Set RLS context
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let cards = sqlx::query_as::<_, AgentFeedCard>(
            r#"
            SELECT id, tenant_id, agent_type AS "agent_type!: AgentType", card_type AS "card_type!: CardType", title, description, proposed_action_payload as "proposed_action_payload: sqlx::types::Json<serde_json::Value>", status AS "status!: CardStatus", created_at, updated_at
            FROM agent_feed_cards
            WHERE tenant_id = $1 AND status = 'Pending'::card_status_enum
            ORDER BY created_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(cards)
    }

    pub async fn resolve_card(&self, tenant_id: Uuid, card_id: Uuid, new_status: CardStatus) -> Result<AgentFeedCard, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Set RLS context
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let card = sqlx::query_as::<_, AgentFeedCard>(
            r#"
            UPDATE agent_feed_cards
            SET status = $1::card_status_enum, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, agent_type AS "agent_type!: AgentType", card_type AS "card_type!: CardType", title, description, proposed_action_payload as "proposed_action_payload: sqlx::types::Json<serde_json::Value>", status AS "status!: CardStatus", created_at, updated_at
            "#
        )
        .bind(new_status)
        .bind(card_id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(card)
    }
}
