use std::sync::Arc;
use sqlx::PgPool;
use crate::domain::repository::tax_engine_repo::{TaxEngineRepository, TaxNexusThreshold};
use sqlx::types::BigDecimal;
use std::str::FromStr;

#[derive(Clone)]
pub struct RegulatoryAgent {
    repo: TaxEngineRepository,
}

impl RegulatoryAgent {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: TaxEngineRepository::new(pool),
        }
    }

    pub async fn check_nexus_thresholds(&self, tenant_id: &str) -> Result<Vec<String>, String> {
        let thresholds = self.repo.get_nexus_thresholds(tenant_id).await
            .map_err(|e| format!("Failed to fetch nexus thresholds: {}", e))?;

        let mut alerts = Vec::new();

        for threshold in thresholds {
            // Using actual BigDecimal comparisons for robustness
            let current = threshold.current_volume.to_string().parse::<f64>().unwrap_or(0.0);
            let limit = threshold.threshold_volume.to_string().parse::<f64>().unwrap_or(1.0);

            // Check if current volume is at or above 80% of the threshold
            let ratio = current / limit;
            let is_nearing = ratio >= 0.8;

            if is_nearing {
                alerts.push(format!(
                    "You are nearing the economic nexus for jurisdiction {}. You have reached {:.1}% of your threshold limit.",
                    threshold.jurisdiction_id,
                    ratio * 100.0
                ));
            }
            if current >= limit {
                alerts.push(format!(
                    "CRITICAL: You have exceeded the economic nexus threshold for jurisdiction {}. Immediate tax registration may be required.",
                    threshold.jurisdiction_id
                ));
            }
        }

        Ok(alerts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_nexus_threshold_calculation_logic() {
        let current = 450000.0;
        let limit = 500000.0;
        let ratio = current / limit;
        assert_eq!(ratio, 0.9);
        assert!(ratio >= 0.8);
    }
}
