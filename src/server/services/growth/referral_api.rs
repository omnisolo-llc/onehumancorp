use rand::RngCore;

pub fn generate_referral_link(user_id: &str) -> Result<String, String> {
    if user_id.is_empty() {
        return Err("userID cannot be empty".to_string());
    }

    let mut rng = rand::thread_rng();
    let bytes: [u8; 8] = {
        let mut buf = [0u8; 8];
        rng.fill_bytes(&mut buf);
        buf
    };
    let referral_code = hex::encode(bytes);

    // Standalone mode specific deep link
    let link = format!(
        "https://ohc.store/join?ref={}&utm_source=standalone_desktop&utm_medium=team_share&inviter={}",
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
        assert!(link.starts_with("https://ohc.store/join?ref="));
        assert!(link.contains("utm_source=standalone_desktop"));
        assert!(link.contains("inviter=user123"));
        
        let err = generate_referral_link("").unwrap_err();
        assert_eq!(err, "userID cannot be empty");
    }
}
