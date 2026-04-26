use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeProfile {
    pub role_id: String,
    pub min_vram_gb: i32,
    pub preferred_gpu_type: String, // e.g., "h100", "a10g"
    pub scheduling_priority: i32,
}

#[derive(Debug, Clone)]
pub struct AffinityScoreResult {
    pub score: i32,
    pub reason: String,
}

pub struct AffinityEngine;

impl AffinityEngine {
    pub fn calculate_score(&self, profile: &ComputeProfile, is_vip: bool, local_weights_cached: bool) -> AffinityScoreResult {
        let mut score = 0;
        let mut reasons = Vec::new();

        // Model Size / VRAM impact
        if profile.min_vram_gb >= 80 {
            score += 100;
            reasons.push("GPU_REQUIRED");
        } else if profile.min_vram_gb > 0 {
            score += 50;
            reasons.push("GPU_PREFERRED");
        } else {
            score += 10;
            reasons.push("CPU_SUFFICIENT");
        }

        // VIP Task Urgency
        if is_vip {
            score += 50;
            reasons.push("VIP_PRIORITY");
        }

        // Locality
        if local_weights_cached && profile.min_vram_gb > 0 {
            score += 25;
            reasons.push("LOCAL_WEIGHTS_CACHED");
        }

        // Profile base priority
        score += profile.scheduling_priority;

        AffinityScoreResult {
            score,
            reason: reasons.join(", "),
        }
    }
}

pub struct QuotaManager;

impl QuotaManager {
    pub fn check_quota(&self, profile: &ComputeProfile, available_vram: i32) -> Result<(), String> {
        if profile.min_vram_gb > available_vram {
            return Err("quota exceeded: min_vram_gb exceeds available VRAM".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affinity_engine_calculate_score() {
        let ae = AffinityEngine;

        // 70B model
        let profile = ComputeProfile {
            role_id: "researcher".to_string(),
            min_vram_gb: 80,
            preferred_gpu_type: "h100".to_string(),
            scheduling_priority: 10,
        };
        let got = ae.calculate_score(&profile, false, false);
        assert_eq!(got.score, 110);
        assert!(got.reason.contains("GPU_REQUIRED"));

        // VIP task with cached weights
        let profile = ComputeProfile {
            role_id: "swe".to_string(),
            min_vram_gb: 24,
            preferred_gpu_type: "a10g".to_string(),
            scheduling_priority: 5,
        };
        let got = ae.calculate_score(&profile, true, true);
        assert_eq!(got.score, 130);
        assert!(got.reason.contains("GPU_PREFERRED"));
        assert!(got.reason.contains("VIP_PRIORITY"));
        assert!(got.reason.contains("LOCAL_WEIGHTS_CACHED"));

        // Small task
        let profile = ComputeProfile {
            role_id: "planner".to_string(),
            min_vram_gb: 0,
            preferred_gpu_type: "".to_string(),
            scheduling_priority: 0,
        };
        let got = ae.calculate_score(&profile, false, false);
        assert_eq!(got.score, 10);
        assert!(got.reason.contains("CPU_SUFFICIENT"));
    }

    #[test]
    fn test_quota_manager_check_quota() {
        let qm = QuotaManager;

        let profile = ComputeProfile {
            role_id: "".to_string(),
            min_vram_gb: 40,
            preferred_gpu_type: "".to_string(),
            scheduling_priority: 0,
        };
        assert!(qm.check_quota(&profile, 80).is_ok());

        let profile = ComputeProfile {
            role_id: "".to_string(),
            min_vram_gb: 80,
            preferred_gpu_type: "".to_string(),
            scheduling_priority: 0,
        };
        assert!(qm.check_quota(&profile, 80).is_ok());

        let profile = ComputeProfile {
            role_id: "".to_string(),
            min_vram_gb: 120,
            preferred_gpu_type: "".to_string(),
            scheduling_priority: 0,
        };
        assert!(qm.check_quota(&profile, 80).is_err());
    }
}
