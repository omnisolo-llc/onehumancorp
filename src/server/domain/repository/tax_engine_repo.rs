use sqlx::{PgPool, Error, Row};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;

#[derive(Debug, Clone)]
pub struct TaxJurisdiction {
    pub id: String,
    pub country_code: String,
    pub region_code: Option<String>,
    pub tax_rate: BigDecimal,
    pub tax_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaxLedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub order_id: String,
    pub jurisdiction_id: String,
    pub taxable_amount: BigDecimal,
    pub tax_amount: BigDecimal,
    pub tax_rate: BigDecimal,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TaxNexusThreshold {
    pub id: String,
    pub tenant_id: String,
    pub jurisdiction_id: String,
    pub current_volume: BigDecimal,
    pub threshold_volume: BigDecimal,
    pub status: String,
}

#[derive(Clone)]
pub struct TaxEngineRepository {
    pool: PgPool,
}

impl TaxEngineRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_jurisdiction(&self, country_code: &str, region_code: Option<&str>) -> Result<Option<TaxJurisdiction>, Error> {
        let query = match region_code {
            Some(_) => "SELECT id, country_code, region_code, tax_rate, tax_type, description FROM ohc_tax_jurisdictions WHERE country_code = $1 AND region_code = $2 LIMIT 1",
            None => "SELECT id, country_code, region_code, tax_rate, tax_type, description FROM ohc_tax_jurisdictions WHERE country_code = $1 AND region_code IS NULL LIMIT 1",
        };

        let mut q = sqlx::query(query).bind(country_code);

        if let Some(r) = region_code {
            q = q.bind(r);
        }

        let row = q.fetch_optional(&self.pool).await?;

        Ok(row.map(|r| TaxJurisdiction {
            id: r.get("id"),
            country_code: r.get("country_code"),
            region_code: r.get("region_code"),
            tax_rate: r.get("tax_rate"),
            tax_type: r.get("tax_type"),
            description: r.get("description"),
        }))
    }

    pub async fn record_ledger_entry(&self, entry: &TaxLedgerEntry) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO ohc_tax_ledgers (id, tenant_id, order_id, jurisdiction_id, taxable_amount, tax_amount, tax_rate, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&entry.id)
        .bind(&entry.tenant_id)
        .bind(&entry.order_id)
        .bind(&entry.jurisdiction_id)
        .bind(&entry.taxable_amount)
        .bind(&entry.tax_amount)
        .bind(&entry.tax_rate)
        .bind(&entry.status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_nexus_thresholds(&self, tenant_id: &str) -> Result<Vec<TaxNexusThreshold>, Error> {
        let rows = sqlx::query(
            "SELECT id, tenant_id, jurisdiction_id, current_volume, threshold_volume, status FROM ohc_tax_nexus_thresholds WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| TaxNexusThreshold {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            jurisdiction_id: r.get("jurisdiction_id"),
            current_volume: r.get("current_volume"),
            threshold_volume: r.get("threshold_volume"),
            status: r.get("status"),
        }).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tax_models() {
        let jurisdiction = TaxJurisdiction {
            id: "us-ca".to_string(),
            country_code: "US".to_string(),
            region_code: Some("CA".to_string()),
            tax_rate: BigDecimal::from_str("0.08").unwrap(),
            tax_type: "SALES".to_string(),
            description: None,
        };
        assert_eq!(jurisdiction.country_code, "US");
        assert_eq!(jurisdiction.region_code.unwrap(), "CA");

        let ledger = TaxLedgerEntry {
            id: "ledge-1".to_string(),
            tenant_id: "tenant-1".to_string(),
            order_id: "order-1".to_string(),
            jurisdiction_id: "us-ca".to_string(),
            taxable_amount: BigDecimal::from_str("100.0").unwrap(),
            tax_amount: BigDecimal::from_str("8.0").unwrap(),
            tax_rate: BigDecimal::from_str("0.08").unwrap(),
            status: "COLLECTED".to_string(),
        };
        assert_eq!(ledger.status, "COLLECTED");
    }
}
