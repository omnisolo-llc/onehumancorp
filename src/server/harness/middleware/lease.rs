use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaseState {
    Offered,
    #[default]
    Active,
    Released,
    Expired,
    Revoked,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct FenceToken {
    pub lease_id: Option<Uuid>,
    pub attempt_id: Option<String>,
    pub generation: i64,
}

impl FenceToken {
    pub fn new(generation: i64) -> Self {
        Self {
            lease_id: None,
            attempt_id: None,
            generation,
        }
    }

    fn for_lease(lease_id: Uuid, attempt_id: &str, generation: i64) -> Self {
        Self {
            lease_id: Some(lease_id),
            attempt_id: Some(attempt_id.to_owned()),
            generation,
        }
    }

    pub fn value(&self) -> String {
        match (&self.lease_id, &self.attempt_id) {
            (Some(lease_id), Some(attempt_id)) => {
                format!("{lease_id}:{attempt_id}:{}", self.generation)
            }
            _ => format!("generation:{}", self.generation),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum FenceError {
    InvalidGeneration,
    StaleGeneration,
    WrongLease,
    WrongAttempt,
    LeaseNotActive,
    Expired,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum LeaseError {
    Terminal,
    GenerationOverflow,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Lease {
    pub lease_id: Uuid,
    pub attempt_id: String,
    pub generation: i64,
    pub state: LeaseState,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
}

impl Lease {
    pub fn active(attempt_id: impl Into<String>, generation: i64) -> Self {
        Self {
            lease_id: Uuid::new_v4(),
            attempt_id: attempt_id.into(),
            generation,
            state: LeaseState::Active,
            issued_at: Utc::now(),
            expires_at: None,
            released_at: None,
        }
    }

    pub fn token(&self) -> FenceToken {
        FenceToken::for_lease(self.lease_id, &self.attempt_id, self.generation)
    }

    pub fn reassign(&mut self) -> Result<(), LeaseError> {
        if !matches!(self.state, LeaseState::Active | LeaseState::Offered) {
            return Err(LeaseError::Terminal);
        }
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(LeaseError::GenerationOverflow)?;
        self.state = LeaseState::Active;
        self.issued_at = Utc::now();
        self.released_at = None;
        Ok(())
    }

    pub fn release(&mut self) {
        if matches!(self.state, LeaseState::Active | LeaseState::Offered) {
            self.state = LeaseState::Released;
            self.released_at = Some(Utc::now());
        }
    }

    pub fn revoke(&mut self) {
        if matches!(self.state, LeaseState::Active | LeaseState::Offered) {
            self.state = LeaseState::Revoked;
            self.released_at = Some(Utc::now());
        }
    }

    pub fn expire_at(&mut self, now: DateTime<Utc>) {
        if matches!(self.state, LeaseState::Active | LeaseState::Offered)
            && self.expires_at.is_some_and(|expires_at| now >= expires_at)
        {
            self.state = LeaseState::Expired;
            self.released_at = Some(now);
        }
    }

    pub fn validate(&self, token: FenceToken) -> Result<(), FenceError> {
        self.validate_at(token, Utc::now())
    }

    pub fn validate_at(&self, token: FenceToken, now: DateTime<Utc>) -> Result<(), FenceError> {
        if self.generation <= 0 || token.generation <= 0 {
            return Err(FenceError::InvalidGeneration);
        }
        if token.generation != self.generation {
            return Err(FenceError::StaleGeneration);
        }
        if token
            .lease_id
            .is_some_and(|lease_id| lease_id != self.lease_id)
        {
            return Err(FenceError::WrongLease);
        }
        if token
            .attempt_id
            .as_deref()
            .is_some_and(|attempt_id| attempt_id != self.attempt_id)
        {
            return Err(FenceError::WrongAttempt);
        }
        if !matches!(self.state, LeaseState::Active) {
            return match self.state {
                LeaseState::Expired => Err(FenceError::Expired),
                _ => Err(FenceError::LeaseNotActive),
            };
        }
        if self.expires_at.is_some_and(|expires_at| now >= expires_at) {
            return Err(FenceError::Expired);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn stale_worker_cannot_mutate_after_reassignment() {
        let mut lease = Lease::active("attempt-1", 7);
        let stale = FenceToken::new(7);
        lease.reassign().unwrap();

        assert_eq!(lease.validate(stale), Err(FenceError::StaleGeneration));
        assert_eq!(lease.validate(lease.token()), Ok(()));
    }

    #[test]
    fn lease_expiry_rejects_writes_at_the_expiry_boundary() {
        let now = Utc.timestamp_opt(1_700_000_000, 0).single().unwrap();
        let mut lease = Lease::active("attempt-1", 1);
        lease.expires_at = Some(now + Duration::seconds(30));
        let token = lease.token();

        assert_eq!(
            lease.validate_at(token.clone(), now + Duration::seconds(29)),
            Ok(())
        );
        lease.expire_at(now + Duration::seconds(30));
        assert_eq!(
            lease.validate_at(token, now + Duration::seconds(30)),
            Err(FenceError::Expired)
        );
        assert_eq!(lease.state, LeaseState::Expired);
    }

    #[test]
    fn lease_generation_is_monotonic_and_terminal_states_do_not_reopen() {
        let mut lease = Lease::active("attempt-1", 4);
        lease.release();
        assert_eq!(lease.state, LeaseState::Released);
        assert_eq!(lease.reassign(), Err(LeaseError::Terminal));

        let mut lease = Lease::active("attempt-2", 9);
        lease.reassign().unwrap();
        assert_eq!(lease.generation, 10);
        lease.reassign().unwrap();
        assert_eq!(lease.generation, 11);
    }

    #[test]
    fn lease_validates_identity_and_rejects_non_active_states() {
        let mut lease = Lease::active("attempt-1", 1);
        let mut wrong_lease = lease.token();
        wrong_lease.lease_id = Some(Uuid::new_v4());
        assert_eq!(lease.validate(wrong_lease), Err(FenceError::WrongLease));

        let mut wrong_attempt = lease.token();
        wrong_attempt.attempt_id = Some("attempt-2".to_owned());
        assert_eq!(lease.validate(wrong_attempt), Err(FenceError::WrongAttempt));

        lease.release();
        lease.release();
        assert_eq!(
            lease.validate(lease.token()),
            Err(FenceError::LeaseNotActive)
        );

        let mut revoked = Lease::active("attempt-3", 1);
        revoked.revoke();
        revoked.revoke();
        assert_eq!(revoked.state, LeaseState::Revoked);
        assert_eq!(
            revoked.validate(revoked.token()),
            Err(FenceError::LeaseNotActive)
        );

        let mut not_expiring = Lease::active("attempt-4", 1);
        not_expiring.expire_at(Utc::now());
        assert_eq!(not_expiring.state, LeaseState::Active);
    }

    #[test]
    fn lease_reassignment_reports_terminal_and_overflow_errors() {
        let mut expired = Lease::active("attempt-1", 1);
        expired.state = LeaseState::Expired;
        assert_eq!(expired.reassign(), Err(LeaseError::Terminal));

        let mut overflow = Lease::active("attempt-2", i64::MAX);
        assert_eq!(overflow.reassign(), Err(LeaseError::GenerationOverflow));
        assert_eq!(overflow.generation, i64::MAX);
    }

    #[test]
    fn lease_fences_reject_non_positive_generations() {
        let lease = Lease::active("attempt-1", 0);
        assert_eq!(
            lease.validate(lease.token()),
            Err(FenceError::InvalidGeneration)
        );

        let lease = Lease::active("attempt-2", 1);
        assert_eq!(
            lease.validate(FenceToken::new(0)),
            Err(FenceError::InvalidGeneration)
        );
    }

    #[test]
    fn fence_tokens_render_both_scoped_and_generation_only_forms() {
        let lease = Lease::active("attempt-1", 3);
        assert!(lease.token().value().contains("attempt-1"));
        assert_eq!(FenceToken::new(3).value(), "generation:3");

        let now = Utc.timestamp_opt(1_700_000_000, 0).single().unwrap();
        let mut expiring = Lease::active("attempt-4", 1);
        expiring.expires_at = Some(now);
        assert_eq!(
            expiring.validate_at(expiring.token(), now),
            Err(FenceError::Expired)
        );
    }
}
