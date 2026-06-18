use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_collective_action(
    tenant_id: &str,
    payload: &Value,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let partner_id = payload.get("partner_id").and_then(|v| v.as_str()).unwrap_or_default();
    if partner_id.is_empty() {
        return Ok(());
    }

    // 1. Create the Collective
    let collective_id = format!("coll-{}", uuid::Uuid::new_v4());
    let collective_name = format!("Collective: {} & {}", tenant_id, partner_id);

    sqlx::query("INSERT INTO ohc_collective (id, tenant_id, name) VALUES ($1, $2, $3)")
        .bind(&collective_id)
        .bind(tenant_id)
        .bind(&collective_name)
        .execute(pool)
        .await?;

    // 2. Add Tenant A (Originator) as ACTIVE
    sqlx::query("INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ($1, $2, 'ACTIVE')")
        .bind(&collective_id)
        .bind(tenant_id)
        .execute(pool)
        .await?;

    // 3. Invite Tenant B (Partner) as PENDING
    sqlx::query("INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ($1, $2, 'PENDING')")
        .bind(&collective_id)
        .bind(partner_id)
        .execute(pool)
        .await?;

    // 4. Create an approval task for Tenant B to join
    let task_id = uuid::Uuid::new_v4().to_string();
    let description = format!("{} invited you to join a Neighborhood Collective!", tenant_id);
    let context = serde_json::json!({
        "feature_type": "neighborhood_invitation",
        "partner_id": tenant_id,
        "collective_id": collective_id,
        "description": description
    });
    let action = serde_json::json!({
        "action_type": "JOIN_COLLECTIVE",
        "collective_id": collective_id
    });

    sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES ($1, $2, 'marketing', $3, $4, 'PENDING_APPROVAL')")
        .bind(&task_id)
        .bind(partner_id)
        .bind(context)
        .bind(action)
        .execute(pool)
        .await?;

    Ok(())
}

#[derive(serde::Serialize)]
pub struct SettlementSummary {
    pub total_owed_cents: i64,
    pub total_due_cents: i64,
    pub pending_settlements_count: i64,
}

pub async fn get_collective_settlement_summary(
    tenant_id: &str,
    pool: &sqlx::PgPool,
) -> Result<SettlementSummary, sqlx::Error> {
    // Total Owed: Points redeemed at OUR shop that were earned elsewhere (originating_tenant_id != us)
    // Actually, in the loyalty mesh spec: "Points earned at Merchant A but spent at Merchant B
    // require a virtual clearinghouse to balance the books".
    // If a customer spends points at MY shop (target_tenant_id = us), then the ORIGINATOR owes ME money.
    // So if target_tenant_id = us, we are DUE money.
    // If originating_tenant_id = us, we OWED money because someone spent points elsewhere that were earned at our shop.

    let due: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(value_cents), 0) FROM ohc_shared_loyalty_ledger WHERE target_tenant_id = $1 AND status = 'PENDING'"
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await?;

    let owed: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(value_cents), 0) FROM ohc_shared_loyalty_ledger WHERE originating_tenant_id = $1 AND status = 'PENDING'"
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await?;

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ohc_shared_loyalty_ledger WHERE (originating_tenant_id = $1 OR target_tenant_id = $1) AND status = 'PENDING'"
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await?;

    Ok(SettlementSummary {
        total_owed_cents: owed,
        total_due_cents: due,
        pending_settlements_count: count,
    })
}

pub async fn handle_join_collective(
    tenant_id: &str,
    payload: &Value,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let collective_id = payload.get("collective_id").and_then(|v| v.as_str()).unwrap_or_default();
    if collective_id.is_empty() {
        return Ok(());
    }

    sqlx::query("UPDATE ohc_collective_member SET status = 'ACTIVE', updated_at = CURRENT_TIMESTAMP WHERE collective_id = $1 AND tenant_id = $2")
        .bind(collective_id)
        .bind(tenant_id)
        .execute(pool)
        .await?;

    Ok(())
}
