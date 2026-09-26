use rand::RngCore;

pub fn generate_referral_link(user_id: &str) -> Result<String, String> {
    if user_id.is_empty() {
        return Err("userID cannot be empty".to_string());
    }
    if user_id.len() > 255 {
        return Err("userID is too long".to_string());
    }

    let mut rng = rand::thread_rng();
    let bytes: [u8; 8] = {
        let mut buf = [0u8; 8];
        rng.fill_bytes(&mut buf);
        buf
    };
    let referral_code = hex::encode(bytes);

    // Trackable growth referral deep link
    let link = format!(
        "https://omnisolo.co/api/v1/growth/referrals/click?target=/onboarding&ref={}&source=standalone_desktop&inviter={}",
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
        assert!(link.starts_with(
            "https://omnisolo.co/api/v1/growth/referrals/click?target=/onboarding&ref="
        ));
        assert!(link.contains("source=standalone_desktop"));
        assert!(link.contains("inviter=user123"));

        let err = generate_referral_link("").unwrap_err();
        assert_eq!(err, "userID cannot be empty");

        let err_long = generate_referral_link(&"a".repeat(256)).unwrap_err();
        assert_eq!(err_long, "userID is too long");
    }
}
