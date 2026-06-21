use sqlx::PgPool;
use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub async fn get_or_create_referral_code(
    pool: &PgPool,
    tenant_id: &str,
    customer_id: &str,
) -> Result<String, String> {
    // Try to fetch existing
    let existing = sqlx::query(
        r#"
        SELECT code FROM referral_codes
        WHERE tenant_id = $1 AND customer_id = $2
        "#
    )
    .bind(tenant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch referral code: {}", e))?;

    if let Some(record) = existing {
        use sqlx::Row;
        return Ok(record.get("code"));
    }

    // Generate new code
    let code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO referral_codes (id, tenant_id, customer_id, code)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (tenant_id, customer_id) DO NOTHING
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(customer_id)
    .bind(&code)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create referral code: {}", e))?;

    // Fetch again to ensure we get the right code in case of race condition
    let record = sqlx::query(
        r#"
        SELECT code FROM referral_codes
        WHERE tenant_id = $1 AND customer_id = $2
        "#
    )
    .bind(tenant_id)
    .bind(customer_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to fetch referral code: {}", e))?;

    use sqlx::Row;
    Ok(record.get("code"))
}

pub async fn track_referral_click(
    pool: &PgPool,
    tenant_id: &str,
    code: &str,
) -> Result<String, String> {
    let referral_code = sqlx::query(
        r#"
        SELECT id FROM referral_codes
        WHERE tenant_id = $1 AND code = $2
        "#
    )
    .bind(tenant_id)
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch referral code: {}", e))?;

    let code_id: String = match referral_code {
        Some(rc) => {
            use sqlx::Row;
            rc.get("id")
        }
        None => return Err("Referral code not found".to_string()),
    };

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO referrals (id, tenant_id, referral_code_id, status)
        VALUES ($1, $2, $3, 'Clicked')
        "#
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(code_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to track referral click: {}", e))?;

    Ok(id)
}

pub async fn update_referral_status(
    pool: &PgPool,
    tenant_id: &str,
    referral_id: &str,
    status: &str,
    referred_customer_id: Option<&str>,
) -> Result<(), String> {

    sqlx::query(
        r#"
        UPDATE referrals
        SET status = $1, referred_customer_id = COALESCE($2, referred_customer_id), updated_at = NOW()
        WHERE id = $3 AND tenant_id = $4
        "#
    )
    .bind(status)
    .bind(referred_customer_id)
    .bind(referral_id)
    .bind(tenant_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to update referral status: {}", e))?;

    Ok(())
}
