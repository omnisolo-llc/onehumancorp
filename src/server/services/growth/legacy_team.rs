use crate::analytics::Tracker;
use crate::services::growth::legacy_repo::{ReferralRepository, GrowthReferral};
use std::collections::HashMap;
use std::sync::Arc;

pub struct TeamService {
    tracker: Arc<Tracker>,
    repo: Arc<ReferralRepository>,
}

impl TeamService {
    pub fn new(tracker: Arc<Tracker>, repo: Arc<ReferralRepository>) -> Self {
        TeamService { tracker, repo }
    }

    pub fn send_team_invite(&self, team_id: &str, inviter_id: &str, invitee_email: &str) -> Result<GrowthReferral, String> {
        if team_id.is_empty() || inviter_id.is_empty() || invitee_email.is_empty() {
            return Err("invalid team invite parameters".to_string());
        }

        let referral = GrowthReferral {
            id: format!("team-ref-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            inviter_id: inviter_id.to_string(),
            invitee_email: invitee_email.to_string(),
            status: "PENDING".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.repo.save_referral(referral.clone())?;

        let mut props = HashMap::new();
        props.insert("team_id".to_string(), team_id.to_string());
        props.insert("inviter_id".to_string(), inviter_id.to_string());
        props.insert("invitee_email".to_string(), invitee_email.to_string());
        props.insert("referral_id".to_string(), referral.id.clone());

        self.tracker.track_event("team_invite_sent", props);

        Ok(referral)
    }

    pub fn accept_team_invite(&self, invite_id: &str, spiffe_id: &str) -> Result<(), String> {
        if invite_id.is_empty() {
            return Err("invalid team invite ID".to_string());
        }

        let mut ref_obj = self.repo.get_referral_by_id(invite_id)?;

        if ref_obj.status == "SIGNED_UP" {
            return Ok(());
        }

        ref_obj.status = "SIGNED_UP".to_string();
        self.repo.save_referral(ref_obj)?;

        let mut props = HashMap::new();
        props.insert("invite_id".to_string(), invite_id.to_string());
        props.insert("spiffe_id".to_string(), spiffe_id.to_string());

        self.tracker.track_event("team_invite_accepted", props);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analytics::Tracker;
    use crate::services::growth::legacy_repo::ReferralRepository;
    use std::sync::Arc;

    #[test]
    fn test_send_team_invite() {
        let tracker = Arc::new(Tracker::new());
        let repo = Arc::new(ReferralRepository::new());
        let service = TeamService::new(tracker, repo);

        let res = service.send_team_invite("team-1", "user-1", "test@example.com");
        assert!(res.is_ok());

        let res = service.send_team_invite("", "", "");
        assert!(res.is_err());
    }

    #[test]
    fn test_accept_team_invite() {
        let tracker = Arc::new(Tracker::new());
        let repo = Arc::new(ReferralRepository::new());
        let service = TeamService::new(tracker, repo.clone());

        let ref_obj = service.send_team_invite("team-1", "user-1", "test@example.com").unwrap();

        let res = service.accept_team_invite(&ref_obj.id, "spiffe://example.org/newuser");
        assert!(res.is_ok());

        let res = service.accept_team_invite("", "");
        assert!(res.is_err());
    }
}
