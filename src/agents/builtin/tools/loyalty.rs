use ohc::agent::AgentActionRequest;
use ohc::agent::service::ToolResult;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

pub async fn execute_loyalty_tool(
    pool: &PgPool,
    tenant_id: Uuid,
    req: &AgentActionRequest,
) -> anyhow::Result<ToolResult> {
    let args: serde_json::Value = serde_json::from_str(&req.tool_arguments)?;

    match req.tool_name.as_str() {
        "apply_points" | "redeem_points" => {
            let customer_id_str = args.get("customer_id").and_then(|v| v.as_str()).unwrap_or_default();
            let amount = args.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("checkout application");

            let customer_id = Uuid::parse_str(customer_id_str).map_err(|_| anyhow::anyhow!("invalid customer_id"))?;

            info!("Applying {} loyalty points for customer {}", amount, customer_id);

            let mut tx = pool.begin().await?;

            let rec = sqlx::query!(
                "SELECT id, points_balance FROM loyalty_wallets WHERE tenant_id = $1 AND customer_id = $2",
                tenant_id, customer_id
            ).fetch_optional(&mut *tx).await?;

            if let Some(row) = rec {
                if row.points_balance < amount {
                    return Err(anyhow::anyhow!("Insufficient points"));
                }

                let new_balance = row.points_balance - amount;

                sqlx::query!(
                    "UPDATE loyalty_wallets SET points_balance = $1, updated_at = NOW() WHERE id = $2",
                    new_balance, row.id
                ).execute(&mut *tx).await?;

                sqlx::query!(
                    "INSERT INTO loyalty_ledger (tenant_id, wallet_id, amount, reason) VALUES ($1, $2, $3, $4)",
                    tenant_id, row.id, -amount, reason
                ).execute(&mut *tx).await?;

                tx.commit().await?;

                Ok(ToolResult {
                    stdout: format!("Successfully applied {} points. New balance is {}.", amount, new_balance),
                    stderr: String::new(),
                    structured_data: format!("{{\"new_balance\": {}}}", new_balance),
                })
            } else {
                Err(anyhow::anyhow!("Wallet not found for customer"))
            }
        },
        "grant_points" => {
            let customer_id_str = args.get("customer_id").and_then(|v| v.as_str()).unwrap_or_default();
            let amount = args.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("loyalty grant");

            let customer_id = Uuid::parse_str(customer_id_str).map_err(|_| anyhow::anyhow!("invalid customer_id"))?;

            info!("Granting {} loyalty points to customer {}", amount, customer_id);

            let mut tx = pool.begin().await?;

            let rec = sqlx::query!(
                "SELECT id, points_balance FROM loyalty_wallets WHERE tenant_id = $1 AND customer_id = $2",
                tenant_id, customer_id
            ).fetch_optional(&mut *tx).await?;

            let (wallet_id, current_balance) = if let Some(row) = rec {
                (row.id, row.points_balance)
            } else {
                let id = Uuid::new_v4();
                sqlx::query!(
                    "INSERT INTO loyalty_wallets (id, tenant_id, customer_id, points_balance) VALUES ($1, $2, $3, $4)",
                    id, tenant_id, customer_id, 0
                ).execute(&mut *tx).await?;
                (id, 0)
            };

            let new_balance = current_balance + amount;

            sqlx::query!(
                "UPDATE loyalty_wallets SET points_balance = $1, updated_at = NOW() WHERE id = $2",
                new_balance, wallet_id
            ).execute(&mut *tx).await?;

            sqlx::query!(
                "INSERT INTO loyalty_ledger (tenant_id, wallet_id, amount, reason) VALUES ($1, $2, $3, $4)",
                tenant_id, wallet_id, amount, reason
            ).execute(&mut *tx).await?;

            tx.commit().await?;

            Ok(ToolResult {
                stdout: format!("Successfully granted {} points. New balance is {}.", amount, new_balance),
                stderr: String::new(),
                structured_data: format!("{{\"new_balance\": {}}}", new_balance),
            })
        },
        "get_wallet_balance" => {
            let customer_id_str = args.get("customer_id").and_then(|v| v.as_str()).unwrap_or_default();
            let customer_id = Uuid::parse_str(customer_id_str).map_err(|_| anyhow::anyhow!("invalid customer_id"))?;

            let rec = sqlx::query!(
                "SELECT points_balance FROM loyalty_wallets WHERE tenant_id = $1 AND customer_id = $2",
                tenant_id, customer_id
            ).fetch_optional(pool).await?;

            let balance = rec.map(|r| r.points_balance).unwrap_or(0);

            Ok(ToolResult {
                stdout: format!("Wallet balance is {}.", balance),
                stderr: String::new(),
                structured_data: format!("{{\"balance\": {}}}", balance),
            })
        },
        _ => Err(anyhow::anyhow!("Unknown loyalty tool: {}", req.tool_name)),
    }
}
