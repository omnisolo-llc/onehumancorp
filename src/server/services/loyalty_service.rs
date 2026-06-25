use crate::domain::loyalty_ledger::{CustomerLoyaltyAccount, LoyaltyTransaction};
use crate::domain::repository::loyalty_repo::LoyaltyRepo;
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, error};

#[derive(Clone)]
pub struct LoyaltyService {
    repo: LoyaltyRepo,
}

impl LoyaltyService {
    pub fn new(repo: LoyaltyRepo) -> Self {
        Self { repo }
    }

    pub async fn handle_payment_event(&self, tenant_id: &str, customer_id: &str, amount_cents: i32) -> Result<(), String> {
        // Find active program for tenant
        let program_opt = self.repo.get_program_by_tenant(tenant_id).await.map_err(|e| e.to_string())?;

        let program = match program_opt {
            Some(p) => p,
            None => {
                info!("No active loyalty program found for tenant {}", tenant_id);
                return Ok(());
            }
        };

        // Basic default logic: 1 point per 100 cents ($1)
        let points_earned = amount_cents / 100;

        if points_earned <= 0 {
            return Ok(());
        }

        // Get or create account
        let mut account_opt = self.repo.get_account(tenant_id, &program.id, customer_id).await.map_err(|e| e.to_string())?;

        let account = match account_opt {
            Some(a) => a,
            None => {
                let new_account = CustomerLoyaltyAccount {
                    id: Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.to_string(),
                    program_id: program.id.clone(),
                    customer_id: customer_id.to_string(),
                    points_balance: 0,
                    punches: 0,
                    tier_name: None,
                    created_at: Some(Utc::now()),
                    updated_at: Some(Utc::now()),
                };
                self.repo.create_account(&new_account).await.map_err(|e| e.to_string())?
            }
        };

        let tx = LoyaltyTransaction {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            account_id: account.id.clone(),
            transaction_type: "earn".to_string(),
            amount: points_earned,
            reason: Some("Payment processed".to_string()),
            created_at: Some(Utc::now()),
        };

        self.repo.add_transaction(&tx).await.map_err(|e| {
            error!("Failed to record loyalty transaction: {}", e);
            e.to_string()
        })?;

        info!("Credited {} points to customer {} for tenant {}", points_earned, customer_id, tenant_id);

        Ok(())
    }
}
