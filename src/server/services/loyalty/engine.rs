use sqlx::PgPool;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

pub async fn create_loyalty_program(
    pool: &PgPool,
    tenant_id: &str,
    name: &str,
    program_type: &str,
    config: JsonValue,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        id,
        tenant_id,
        name,
        program_type,
        config
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create loyalty program: {}", e))?;

    Ok(id)
}

pub async fn get_loyalty_programs(pool: &PgPool, tenant_id: &str) -> Result<Vec<JsonValue>, String> {
    let records = sqlx::query!(
        r#"
        SELECT id, name, program_type, config, is_active
        FROM loyalty_programs
        WHERE tenant_id = $1
        "#,
        tenant_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch programs: {}", e))?;

    let mut programs = Vec::new();
    for rec in records {
        programs.push(serde_json::json!({
            "id": rec.id,
            "name": rec.name,
            "program_type": rec.program_type,
            "config": rec.config,
            "is_active": rec.is_active
        }));
    }

    Ok(programs)
}

pub async fn enroll_customer(
    pool: &PgPool,
    tenant_id: &str,
    program_id: &str,
    customer_id: &str,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO customer_loyalty_accounts (id, tenant_id, program_id, customer_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (tenant_id, program_id, customer_id) DO NOTHING
        "#,
        id,
        tenant_id,
        program_id,
        customer_id
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to enroll customer: {}", e))?;

    let record = sqlx::query!(
        r#"
        SELECT id FROM customer_loyalty_accounts
        WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3
        "#,
        tenant_id,
        program_id,
        customer_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to fetch account ID: {}", e))?;

    Ok(record.id)
}

pub async fn get_customer_account(
    pool: &PgPool,
    tenant_id: &str,
    program_id: &str,
    customer_id: &str,
) -> Result<Option<JsonValue>, String> {
    let record = sqlx::query!(
        r#"
        SELECT id, points_balance, punches, tier_name
        FROM customer_loyalty_accounts
        WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3
        "#,
        tenant_id,
        program_id,
        customer_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch customer account: {}", e))?;

    if let Some(rec) = record {
        Ok(Some(serde_json::json!({
            "id": rec.id,
            "points_balance": rec.points_balance,
            "punches": rec.punches,
            "tier_name": rec.tier_name
        })))
    } else {
        Ok(None)
    }
}

pub async fn create_reward(
    pool: &PgPool,
    tenant_id: &str,
    program_id: &str,
    name: &str,
    description: Option<&str>,
    cost_in_points: i32,
    reward_type: &str,
    reward_value: JsonValue,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO loyalty_rewards (id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        id,
        tenant_id,
        program_id,
        name,
        description,
        cost_in_points,
        reward_type,
        reward_value
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create reward: {}", e))?;

    Ok(id)
}

pub async fn get_rewards(pool: &PgPool, tenant_id: &str, program_id: &str) -> Result<Vec<JsonValue>, String> {
    let records = sqlx::query!(
        r#"
        SELECT id, name, description, cost_in_points, reward_type, reward_value, is_active
        FROM loyalty_rewards
        WHERE tenant_id = $1 AND program_id = $2
        "#,
        tenant_id,
        program_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch rewards: {}", e))?;

    let mut rewards = Vec::new();
    for rec in records {
        rewards.push(serde_json::json!({
            "id": rec.id,
            "name": rec.name,
            "description": rec.description,
            "cost_in_points": rec.cost_in_points,
            "reward_type": rec.reward_type,
            "reward_value": rec.reward_value,
            "is_active": rec.is_active
        }));
    }

    Ok(rewards)
}

pub async fn record_transaction(
    pool: &PgPool,
    tenant_id: &str,
    account_id: &str,
    tx_type: &str, // 'earn', 'redeem', 'adjust'
    amount: i32,
    reason: Option<&str>,
    orchestrator: Option<Arc<DepartmentOrchestrator>>,
) -> Result<(), String> {
    let id = Uuid::new_v4().to_string();

    let mut tx = pool.begin().await.map_err(|e| format!("Failed to begin transaction: {}", e))?;

    let delta = match tx_type {
        "earn" => amount.abs(),
        "redeem" => -amount.abs(),
        "adjust" => amount,
        _ => return Err("Invalid transaction_type".to_string()),
    };

    let current = sqlx::query!(
        r#"
        SELECT points_balance FROM customer_loyalty_accounts
        WHERE id = $1 AND tenant_id = $2
        FOR UPDATE
        "#,
        account_id,
        tenant_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| format!("Failed to lock account: {}", e))?;

    if let Some(record) = current {
        if record.points_balance + delta < 0 {
            return Err("Insufficient points balance for redemption".to_string());
        }
    } else {
        return Err("Account not found".to_string());
    }

    sqlx::query!(
        r#"
        INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, amount, reason)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        tenant_id,
        account_id,
        tx_type,
        delta,
        reason
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to insert transaction: {}", e))?;

    sqlx::query!(
        r#"
        UPDATE customer_loyalty_accounts
        SET points_balance = points_balance + $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $2 AND tenant_id = $3
        "#,
        delta,
        account_id,
        tenant_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to update account balance: {}", e))?;

    tx.commit().await.map_err(|e| format!("Failed to commit transaction: {}", e))?;

    if let Some(orch) = orchestrator {
        let payload = serde_json::json!({
            "account_id": account_id,
            "transaction_type": tx_type,
            "amount": amount,
            "reason": reason
        });

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_type: "loyalty.points_awarded".to_string(),
            payload,
        };

        // Fire and forget
        let orch_clone = orch.clone();
        tokio::spawn(async move {
            let _ = orch_clone.dispatch_event(event).await;
        });
    }

    Ok(())
}
