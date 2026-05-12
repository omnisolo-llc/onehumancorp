pub mod referral;
pub mod share;
pub mod social;
pub mod email;
pub mod funnel;
pub mod storefront;
pub mod notifications;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referral_program() {
        let rp = referral::ReferralProgram::new("test_user");
        assert_eq!(rp.share_link, "https://ohc.app/ref/test_user");
        assert_eq!(rp.invites_sent, 0);
    }
}
