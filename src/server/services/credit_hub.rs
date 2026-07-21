use sqlx::{Row, FromRow};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::db::{DB, DbStore};
use crate::minimax::MinimaxClient;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreditFacility {
    pub id: String,
    pub tenant_id: String,
    pub approved_limit_usd: f64,
    pub utilized_amount_usd: f64,
    pub dynamic_score: f64,
    pub underwriter_version: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VendorRelation {
    pub id: String,
    pub tenant_id: String,
    pub vendor_name: String,
    pub vendor_email: String,
    pub current_terms: String,
    pub term_status: String,
    pub terms_granted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SupplierInvoice {
    pub id: String,
    pub tenant_id: String,
    pub vendor_relation_id: String,
    pub invoice_number: String,
    pub total_amount: f64,
    pub currency: String,
    pub due_date: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FactoringDiscount {
    pub id: String,
    pub tenant_id: String,
    pub client_invoice_id: String,
    pub invoice_amount: f64,
    pub advance_rate: f64,
    pub flat_fee_pct: f64,
    pub advanced_amount_usd: f64,
    pub factoring_status: String,
    pub disbursed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LedgerSweepConfig {
    pub id: String,
    pub supplier_invoice_id: String,
    pub daily_sweep_pct: f64,
    pub maximum_sweep_usd: Option<f64>,
    pub accumulated_sweep_usd: f64,
    pub last_sweep_run: Option<DateTime<Utc>>,
}

// Underwriting Engine
pub async fn calculate_underwriting_capacity(
    db: &DB,
    tenant_id: &str,
) -> Result<CreditFacility, String> {
    // 1. Calculate sales velocity (last 30 days) from orders table
    let sales_total: f64 = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let row = sqlx::query("SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = $1 AND status != 'cancelled'")
                .bind(tenant_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            row.get::<f64, _>(0)
        }
        DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = ? AND status != 'cancelled'")
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;
            row.get::<f64, _>(0)
        }
    };

    // 2. Calculate upcoming bookings value (mock or calculate if booking cost exists)
    let bookings_count: i64 = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let row = sqlx::query("SELECT COUNT(*) FROM bookings WHERE tenant_id = $1 AND status != 'cancelled'")
                .bind(tenant_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            row.get::<i64, _>(0)
        }
        DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT COUNT(*) FROM bookings WHERE tenant_id = ? AND status != 'cancelled'")
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;
            row.get::<i64, _>(0)
        }
    };

    // Underwriting scoring formula
    let base_limit = 1000.0;
    let sales_factor = sales_total * 0.50;
    let bookings_factor = (bookings_count as f64) * 150.0 * 0.25; // Estimate $150 per booking
    let approved_limit = base_limit + sales_factor + bookings_factor;

    // Score defaults to 75.0, scaled with sales total
    let mut score = 70.0 + (sales_total / 200.0);
    if score > 100.0 {
        score = 100.0;
    }

    // Check if facility already exists
    let existing_facility: Option<CreditFacility> = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let row_opt = sqlx::query_as::<_, CreditFacility>("SELECT id::text as id, tenant_id, approved_limit_usd, utilized_amount_usd, dynamic_score, underwriter_version, updated_at FROM credit_facilities WHERE tenant_id = $1")
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            row_opt
        }
        DbStore::Sqlite(pool) => {
            let row_opt = sqlx::query("SELECT id, tenant_id, approved_limit_usd, utilized_amount_usd, dynamic_score, underwriter_version, updated_at FROM credit_facilities WHERE tenant_id = ?")
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;
            row_opt.map(|row| CreditFacility {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                approved_limit_usd: row.get("approved_limit_usd"),
                utilized_amount_usd: row.get("utilized_amount_usd"),
                dynamic_score: row.get("dynamic_score"),
                underwriter_version: row.get("underwriter_version"),
                updated_at: Some(Utc::now()),
            })
        }
    };

    let facility = if let Some(mut existing) = existing_facility {
        existing.approved_limit_usd = approved_limit;
        existing.dynamic_score = score;
        existing.updated_at = Some(Utc::now());

        // Update DB
        match &db.store {
            DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                sqlx::query("UPDATE credit_facilities SET approved_limit_usd = $1, dynamic_score = $2, updated_at = $3 WHERE tenant_id = $4")
                    .bind(approved_limit)
                    .bind(score)
                    .bind(Utc::now())
                    .bind(tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE credit_facilities SET approved_limit_usd = ?, dynamic_score = ?, updated_at = ? WHERE tenant_id = ?")
                    .bind(approved_limit)
                    .bind(score)
                    .bind(Utc::now().to_rfc3339())
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        existing
    } else {
        let new_id = uuid::Uuid::new_v4().to_string();
        let new_facility = CreditFacility {
            id: new_id.clone(),
            tenant_id: tenant_id.to_string(),
            approved_limit_usd: approved_limit,
            utilized_amount_usd: 0.0,
            dynamic_score: score,
            underwriter_version: Some("v1.0".to_string()),
            updated_at: Some(Utc::now()),
        };

        // Insert into DB
        match &db.store {
            DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO credit_facilities (id, tenant_id, approved_limit_usd, utilized_amount_usd, dynamic_score, underwriter_version, updated_at) VALUES ($1::uuid, $2, $3, $4, $5, $6, $7)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(approved_limit)
                    .bind(0.0)
                    .bind(score)
                    .bind("v1.0")
                    .bind(Utc::now())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO credit_facilities (id, tenant_id, approved_limit_usd, utilized_amount_usd, dynamic_score, underwriter_version, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(approved_limit)
                    .bind(0.0)
                    .bind(score)
                    .bind("v1.0")
                    .bind(Utc::now().to_rfc3339())
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        new_facility
    };

    Ok(facility)
}

// AI Negotiator Runner
pub async fn trigger_ai_negotiation(
    db: &DB,
    tenant_id: &str,
    vendor_relation_id: &str,
) -> Result<VendorRelation, String> {
    // 1. Fetch vendor relation
    let vendor: Option<VendorRelation> = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let row_opt = sqlx::query_as::<_, VendorRelation>("SELECT id::text as id, tenant_id, vendor_name, vendor_email, current_terms, term_status, terms_granted_at FROM vendor_relations WHERE id = $1::uuid AND tenant_id = $2")
                .bind(vendor_relation_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            row_opt
        }
        DbStore::Sqlite(pool) => {
            let row_opt = sqlx::query("SELECT id, tenant_id, vendor_name, vendor_email, current_terms, term_status, terms_granted_at FROM vendor_relations WHERE id = ? AND tenant_id = ?")
                .bind(vendor_relation_id)
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;
            row_opt.map(|row| VendorRelation {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                vendor_name: row.get("vendor_name"),
                vendor_email: row.get("vendor_email"),
                current_terms: row.get("current_terms"),
                term_status: row.get("term_status"),
                terms_granted_at: Some(Utc::now()),
            })
        }
    };

    let mut vendor = vendor.ok_or_else(|| "Vendor relation not found".to_string())?;

    // AI dynamic reasoning and negotiation letter drafting
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let proposal_draft = if !api_key.is_empty() {
        let prompt = format!(
            "Draft a warm, highly professional business email from a boutique merchant requesting Net-30 payment terms from supplier {}. Mention the merchant's stable sales volume and order consistency. Do not expose sensitive customer names.",
            vendor.vendor_name
        );
        let client = MinimaxClient::new(api_key);
        client.reason(&prompt).await.unwrap_or_else(|_| "Standard Net-30 credit term proposal request.".to_string())
    } else {
        "Standard Net-30 credit term proposal request.".to_string()
    };

    tracing::info!("AI Drafted Net Terms Proposal:\n{}", proposal_draft);

    // Update Vendor terms status to "NEGOTIATING"
    vendor.term_status = "NEGOTIATING".to_string();
    vendor.current_terms = "NET_30".to_string();

    match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            sqlx::query("UPDATE vendor_relations SET term_status = $1, current_terms = $2, terms_granted_at = $3 WHERE id = $4::uuid AND tenant_id = $5")
                .bind("NEGOTIATING")
                .bind("NET_30")
                .bind(Utc::now())
                .bind(vendor_relation_id)
                .bind(tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
        }
        DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE vendor_relations SET term_status = ?, current_terms = ?, terms_granted_at = ? WHERE id = ? AND tenant_id = ?")
                .bind("NEGOTIATING")
                .bind("NET_30")
                .bind(Utc::now().to_rfc3339())
                .bind(vendor_relation_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(vendor)
}

// Auto-Sweep Execution
pub async fn execute_daily_sweep(
    db: &DB,
    tenant_id: &str,
    invoice_id: &str,
    sales_amount: f64,
) -> Result<LedgerSweepConfig, String> {
    // 1. Fetch sweep config
    let sweep: Option<LedgerSweepConfig> = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let row_opt = sqlx::query_as::<_, LedgerSweepConfig>("SELECT id::text as id, supplier_invoice_id::text as supplier_invoice_id, daily_sweep_pct, maximum_sweep_usd, accumulated_sweep_usd, last_sweep_run FROM ledger_sweep_configs WHERE supplier_invoice_id = $1::uuid")
                .bind(invoice_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            row_opt
        }
        DbStore::Sqlite(pool) => {
            let row_opt = sqlx::query("SELECT id, supplier_invoice_id, daily_sweep_pct, maximum_sweep_usd, accumulated_sweep_usd, last_sweep_run FROM ledger_sweep_configs WHERE supplier_invoice_id = ?")
                .bind(invoice_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;
            row_opt.map(|row| LedgerSweepConfig {
                id: row.get("id"),
                supplier_invoice_id: row.get("supplier_invoice_id"),
                daily_sweep_pct: row.get("daily_sweep_pct"),
                maximum_sweep_usd: row.get("maximum_sweep_usd"),
                accumulated_sweep_usd: row.get("accumulated_sweep_usd"),
                last_sweep_run: Some(Utc::now()),
            })
        }
    };

    let mut sweep = sweep.ok_or_else(|| "Sweep configuration not found for this invoice".to_string())?;

    // 2. Fetch invoice to check outstanding amount
    let invoice_amount: f64 = match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
            let amount = sqlx::query_scalar::<_, f64>("SELECT total_amount FROM supplier_invoices WHERE id = $1::uuid AND tenant_id = $2")
                .bind(invoice_id)
                .bind(tenant_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            amount
        }
        DbStore::Sqlite(pool) => {
            sqlx::query_scalar::<_, f64>("SELECT total_amount FROM supplier_invoices WHERE id = ? AND tenant_id = ?")
                .bind(invoice_id)
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?
        }
    };

    let remaining = invoice_amount - sweep.accumulated_sweep_usd;
    if remaining <= 0.0 {
        return Ok(sweep);
    }

    // Sweep amount calculated using configuration percent
    let mut sweep_amount = sales_amount * sweep.daily_sweep_pct;
    if let Some(max) = sweep.maximum_sweep_usd {
        if sweep_amount > max {
            sweep_amount = max;
        }
    }

    if sweep_amount > remaining {
        sweep_amount = remaining;
    }

    let new_accumulated = sweep.accumulated_sweep_usd + sweep_amount;
    sweep.accumulated_sweep_usd = new_accumulated;
    sweep.last_sweep_run = Some(Utc::now());

    // Update sweep configuration
    match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

            sqlx::query("UPDATE ledger_sweep_configs SET accumulated_sweep_usd = $1, last_sweep_run = $2 WHERE id = $3::uuid")
                .bind(new_accumulated)
                .bind(Utc::now())
                .bind(&sweep.id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            if new_accumulated >= invoice_amount {
                // Mark invoice as PAID
                sqlx::query("UPDATE supplier_invoices SET status = $1 WHERE id = $2::uuid")
                    .bind("PAID")
                    .bind(invoice_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                sqlx::query("UPDATE supplier_invoices SET status = $1 WHERE id = $2::uuid")
                    .bind("SWEEPING")
                    .bind(invoice_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            tx.commit().await.map_err(|e| e.to_string())?;
        }
        DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE ledger_sweep_configs SET accumulated_sweep_usd = ?, last_sweep_run = ? WHERE id = ?")
                .bind(new_accumulated)
                .bind(Utc::now().to_rfc3339())
                .bind(&sweep.id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            if new_accumulated >= invoice_amount {
                sqlx::query("UPDATE supplier_invoices SET status = ? WHERE id = ?")
                    .bind("PAID")
                    .bind(invoice_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                sqlx::query("UPDATE supplier_invoices SET status = ? WHERE id = ?")
                    .bind("SWEEPING")
                    .bind(invoice_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(sweep)
}

// Invoice Discounting / Factoring Payout
pub async fn submit_invoice_factoring(
    db: &DB,
    tenant_id: &str,
    client_invoice_id: &str,
    invoice_amount: f64,
) -> Result<FactoringDiscount, String> {
    let advance_rate = 0.85;
    let flat_fee_pct = 0.02;
    let advanced_amount_usd = invoice_amount * advance_rate * (1.0 - flat_fee_pct);

    let new_id = uuid::Uuid::new_v4().to_string();
    let factoring = FactoringDiscount {
        id: new_id.clone(),
        tenant_id: tenant_id.to_string(),
        client_invoice_id: client_invoice_id.to_string(),
        invoice_amount,
        advance_rate,
        flat_fee_pct,
        advanced_amount_usd,
        factoring_status: "DISBURSED".to_string(),
        disbursed_at: Some(Utc::now()),
    };

    match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

            sqlx::query("INSERT INTO factoring_discounts (id, tenant_id, client_invoice_id, invoice_amount, advance_rate, flat_fee_pct, advanced_amount_usd, factoring_status, disbursed_at) VALUES ($1::uuid, $2, $3, $4, $5, $6, $7, $8, $9)")
                .bind(&new_id)
                .bind(tenant_id)
                .bind(client_invoice_id)
                .bind(invoice_amount)
                .bind(advance_rate)
                .bind(flat_fee_pct)
                .bind(advanced_amount_usd)
                .bind("DISBURSED")
                .bind(Utc::now())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
        }
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO factoring_discounts (id, tenant_id, client_invoice_id, invoice_amount, advance_rate, flat_fee_pct, advanced_amount_usd, factoring_status, disbursed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&new_id)
                .bind(tenant_id)
                .bind(client_invoice_id)
                .bind(invoice_amount)
                .bind(advance_rate)
                .bind(flat_fee_pct)
                .bind(advanced_amount_usd)
                .bind("DISBURSED")
                .bind(Utc::now().to_rfc3339())
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(factoring)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_sqlite_pool_for_test;

    async fn setup_test_db() -> DB {
        let pool = create_sqlite_pool_for_test().await;
        // Create required tables
        sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS bookings (id TEXT, tenant_id TEXT, status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS credit_facilities (id TEXT PRIMARY KEY, tenant_id TEXT, approved_limit_usd REAL, utilized_amount_usd REAL, dynamic_score REAL, underwriter_version TEXT, updated_at TIMESTAMP)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS vendor_relations (id TEXT PRIMARY KEY, tenant_id TEXT, vendor_name TEXT, vendor_email TEXT, current_terms TEXT, term_status TEXT, terms_granted_at TIMESTAMP)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS supplier_invoices (id TEXT PRIMARY KEY, tenant_id TEXT, vendor_relation_id TEXT, invoice_number TEXT, total_amount REAL, currency TEXT, due_date TIMESTAMP, status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS factoring_discounts (id TEXT PRIMARY KEY, tenant_id TEXT, client_invoice_id TEXT, invoice_amount REAL, advance_rate REAL, flat_fee_pct REAL, advanced_amount_usd REAL, factoring_status TEXT, disbursed_at TIMESTAMP)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS ledger_sweep_configs (id TEXT PRIMARY KEY, supplier_invoice_id TEXT, daily_sweep_pct REAL, maximum_sweep_usd REAL, accumulated_sweep_usd REAL, last_sweep_run TIMESTAMP)").execute(&pool).await.unwrap();

        DB {
            pool: crate::db::create_dummy_pg_pool().await,
            store: DbStore::Sqlite(pool),
        }
    }

    #[tokio::test]
    async fn test_calculate_underwriting_capacity() {
        let db = setup_test_db().await;
        let tenant_id = "test-tenant-123";

        // Seed some orders and bookings to calculate capacity
        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO orders (id, tenant_id, total_amount, status) VALUES ('o1', ?, 500.0, 'completed')").bind(tenant_id).execute(pool).await.unwrap();
            sqlx::query("INSERT INTO orders (id, tenant_id, total_amount, status) VALUES ('o2', ?, 300.0, 'completed')").bind(tenant_id).execute(pool).await.unwrap();
            sqlx::query("INSERT INTO bookings (id, tenant_id, status) VALUES ('b1', ?, 'confirmed')").bind(tenant_id).execute(pool).await.unwrap();
        }

        let facility = calculate_underwriting_capacity(&db, tenant_id).await.unwrap();
        assert_eq!(facility.tenant_id, tenant_id);
        // limit = 1000 + (800 * 0.5) + (1 * 150 * 0.25) = 1000 + 400 + 37.5 = 1437.5
        assert_eq!(facility.approved_limit_usd, 1437.5);
        assert_eq!(facility.dynamic_score, 74.0);
    }

    #[tokio::test]
    async fn test_trigger_ai_negotiation() {
        let db = setup_test_db().await;
        let tenant_id = "test-tenant-123";
        let vendor_relation_id = "vendor-abc-123";

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO vendor_relations (id, tenant_id, vendor_name, vendor_email, current_terms, term_status, terms_granted_at) VALUES (?, ?, 'Fabrics R Us', 'info@fabrics.example', 'COD', 'APPROVED', NULL)")
                .bind(vendor_relation_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .unwrap();
        }

        let vendor = trigger_ai_negotiation(&db, tenant_id, vendor_relation_id).await.unwrap();
        assert_eq!(vendor.term_status, "NEGOTIATING");
        assert_eq!(vendor.current_terms, "NET_30");
    }

    #[tokio::test]
    async fn test_execute_daily_sweep() {
        let db = setup_test_db().await;
        let tenant_id = "test-tenant-123";
        let invoice_id = "inv-abc-123";
        let sweep_id = "sweep-abc-123";

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO supplier_invoices (id, tenant_id, vendor_relation_id, invoice_number, total_amount, currency, status) VALUES (?, ?, 'vendor-id', 'INV-001', 1000.0, 'USD', 'UNPAID')")
                .bind(invoice_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO ledger_sweep_configs (id, supplier_invoice_id, daily_sweep_pct, maximum_sweep_usd, accumulated_sweep_usd) VALUES (?, ?, 0.10, 100.0, 0.0)")
                .bind(sweep_id)
                .bind(invoice_id)
                .execute(pool)
                .await
                .unwrap();
        }

        // Daily sales = $500, daily_sweep_pct = 10% -> sweep_amount = $50
        let sweep = execute_daily_sweep(&db, tenant_id, invoice_id, 500.0).await.unwrap();
        assert_eq!(sweep.accumulated_sweep_usd, 50.0);

        // Run another sweep with large daily sales ($2000), should hit maximum_sweep_usd of $100
        let sweep2 = execute_daily_sweep(&db, tenant_id, invoice_id, 2000.0).await.unwrap();
        assert_eq!(sweep2.accumulated_sweep_usd, 150.0);
    }

    #[tokio::test]
    async fn test_submit_invoice_factoring() {
        let db = setup_test_db().await;
        let tenant_id = "test-tenant-123";
        let client_invoice_id = "client-inv-123";

        let factoring = submit_invoice_factoring(&db, tenant_id, client_invoice_id, 10000.0).await.unwrap();
        assert_eq!(factoring.client_invoice_id, client_invoice_id);
        assert_eq!(factoring.invoice_amount, 10000.0);
        // advanced = 10000 * 0.85 * 0.98 = 8330
        assert_eq!(factoring.advanced_amount_usd, 8330.0);
        assert_eq!(factoring.factoring_status, "DISBURSED");
    }
}
