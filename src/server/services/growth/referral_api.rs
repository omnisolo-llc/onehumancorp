use super::referrals::ReferralTracker;
use std::sync::OnceLock;

static TRACKER: OnceLock<ReferralTracker> = OnceLock::new();

pub fn get_tracker() -> &'static ReferralTracker {
    TRACKER.get_or_init(|| ReferralTracker::new())
}

pub fn generate_referral_link(user_id: &str) -> Result<String, String> {
    if user_id.is_empty() {
        return Err("userID cannot be empty".to_string());
    }

    let tracker = get_tracker();
    let referral_code = tracker.generate_referral_code(user_id);

    // Standalone mode specific deep link
    let link = format!(
        "ohc://join?ref={}&utm_source=standalone_desktop&utm_medium=team_share&inviter={}",
        referral_code, user_id
    );
    Ok(link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_referral_link() {
        let link = generate_referral_link("user123").unwrap();
        assert!(link.starts_with("ohc://join?ref="));
        assert!(link.contains("utm_source=standalone_desktop"));
        assert!(link.contains("inviter=user123"));
        
        let err = generate_referral_link("").unwrap_err();
        assert_eq!(err, "userID cannot be empty");
    }
}
