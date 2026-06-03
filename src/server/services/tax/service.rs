use std::sync::Arc;
use crate::db::DB;
use crate::db::DbStore;


use chrono::Utc;
use uuid::Uuid;

pub struct TaxComputationEngine {
    repo: TaxRepository,
}

impl TaxComputationEngine {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: TaxRepository::new(db),
        }
    }

    /// Evaluates tax based on the provided location details. Includes simple edge-caching mechanics
    /// by consulting the DB, which would ideally be pre-warmed for common locations in a full offline-sync model.
    pub async fn calculate_tax(
        &self,
        tenant_id: &str,
        transaction_id: &str,
        amount: f64,
        country_code: &str,
        state_code: Option<&str>,
        zip_code: Option<&str>,
        product_category: Option<&str>,
    ) -> Result<TaxLedgerEntry, String> {
        let jurisdiction = self.repo.get_jurisdiction(country_code, state_code, zip_code).await?;

        let (jurisdiction_id, base_rate) = match jurisdiction {
            Some(j) => (j.id, j.base_rate),
            None => {
                // If not found, fallback to 0% but log a default jurisdiction.
                // A real system would use a dynamic provider here if the DB cache missed.
                (format!("default-{}", country_code), 0.0)
            }
        };

        // Simplified rule evaluation: in reality, product_category might adjust the rate
        // based on rules in the JSONB field.
        let mut final_rate = base_rate;
        if let Some(category) = product_category {
            if category.eq_ignore_ascii_case("digital") || category.eq_ignore_ascii_case("software") {
                // Example of logic modifying rate based on category
                final_rate = base_rate * 0.8; // Example rule: digital goods get a slight reduction in some jurisdictions
            }
        }

        let tax_amount = amount * final_rate;

        let entry = TaxLedgerEntry {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            transaction_id: transaction_id.to_string(),
            jurisdiction_id,
            taxable_amount: amount,
            tax_amount,
            product_category: product_category.map(|s| s.to_string()),
            collected_at: Some(Utc::now()),
        };

        self.repo.record_tax_ledger(entry.clone()).await?;

        Ok(entry)
    }

    pub async fn get_tenant_tax_ledgers(&self, tenant_id: &str) -> Result<Vec<TaxLedgerEntry>, String> {
        self.repo.get_tenant_tax_ledgers(tenant_id).await
    }
}

impl TaxComputationEngine {
    pub async fn evaluate_compliance_thresholds(&self, tenant_id: &str) -> Result<Option<String>, String> {
        let ledgers = self.get_tenant_tax_ledgers(tenant_id).await?;

        let mut total_liability = 0.0;
        let mut jurisdiction_totals = std::collections::HashMap::new();

        for entry in ledgers {
            total_liability += entry.tax_amount;
            *jurisdiction_totals.entry(entry.jurisdiction_id).or_insert(0.0) += entry.tax_amount;
        }

        // Example threshold logic simulating the Regulatory AI Agent
        // Ideally, these thresholds would come from a dynamic configuration or ML model.
        let mut alerts = Vec::new();

        for (jurisdiction, total) in jurisdiction_totals {
            if total > 5000.0 {
                alerts.push(format!("You are nearing the economic nexus for {} sales tax (Liability: ${:.2})", jurisdiction, total));
            }
        }

        if !alerts.is_empty() {
            Ok(Some(alerts.join(" | ")))
        } else {
            Ok(None)
        }
    }
}
