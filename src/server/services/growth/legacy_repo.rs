use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthReferral {
    pub id: String,
    pub inviter_id: String,
    pub invitee_email: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralStats {
    pub invites_sent: usize,
    pub signups: usize,
    pub reward_tier: String,
}

pub struct ReferralRepository {
    referrals: RwLock<HashMap<String, GrowthReferral>>,
}

impl ReferralRepository {
    pub fn new() -> Self {
        ReferralRepository {
            referrals: RwLock::new(HashMap::new()),
        }
    }

    pub fn save_referral(&self, mut referral: GrowthReferral) -> Result<(), String> {
        let now = Utc::now();
        if referral.created_at == DateTime::<Utc>::MIN_UTC {
             referral.created_at = now;
        }
        referral.updated_at = now;

        let mut referrals = self.referrals.write().map_err(|e| e.to_string())?;
        referrals.insert(referral.id.clone(), referral);
        Ok(())
    }

    pub fn get_referral_by_id(&self, referral_id: &str) -> Result<GrowthReferral, String> {
        let referrals = self.referrals.read().map_err(|e| e.to_string())?;
        referrals.get(referral_id).cloned().ok_or_else(|| "referral not found".to_string())
    }

    pub fn get_referrals_by_inviter(&self, inviter_id: &str) -> Result<Vec<GrowthReferral>, String> {
        let referrals = self.referrals.read().map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        for ref_obj in referrals.values() {
            if ref_obj.inviter_id == inviter_id {
                results.push(ref_obj.clone());
            }
        }
        Ok(results)
    }

    pub fn get_stats(&self, inviter_id: &str) -> Result<ReferralStats, String> {
        let referrals = self.get_referrals_by_inviter(inviter_id)?;

        let mut stats = ReferralStats {
            invites_sent: referrals.len(),
            signups: 0,
            reward_tier: "Bronze".to_string(),
        };

        for ref_obj in &referrals {
            if ref_obj.status == "SIGNED_UP" {
                stats.signups += 1;
            }
        }

        if stats.signups >= 50 {
            stats.reward_tier = "Platinum".to_string();
        } else if stats.signups >= 20 {
            stats.reward_tier = "Gold".to_string();
        } else if stats.signups >= 5 {
            stats.reward_tier = "Silver".to_string();
        }

        Ok(stats)
    }

    pub fn get_all_referrals(&self) -> Result<Vec<GrowthReferral>, String> {
        let referrals = self.referrals.read().map_err(|e| e.to_string())?;
        Ok(referrals.values().cloned().collect())
    }

    pub fn get_viral_coefficient(&self) -> Result<f64, String> {
        let referrals = self.get_all_referrals()?;
        if referrals.is_empty() {
            return Ok(0.0);
        }

        let mut unique_inviters = std::collections::HashSet::new();
        let mut signed_up_count = 0;

        for ref_obj in &referrals {
            unique_inviters.insert(ref_obj.inviter_id.clone());
            if ref_obj.status == "SIGNED_UP" {
                signed_up_count += 1;
            }
        }

        if unique_inviters.is_empty() {
            return Ok(0.0);
        }

        Ok(signed_up_count as f64 / unique_inviters.len() as f64)
    }
}

impl Default for ReferralRepository {
    fn default() -> Self {
        Self::new()
    }
}
