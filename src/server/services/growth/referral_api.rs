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
        "ohc://join?ref={}&utm_source=standalone_desktop&utm_medium=team_share&inviter={}",
        referral_code, user_id
    );
    Ok(link)
}

pub fn generate_team_invite_link(team_id: &str, inviter_id: &str) -> Result<String, String> {
    if team_id.is_empty() || inviter_id.is_empty() {
        return Err("team_id and inviter_id cannot be empty".to_string());
    }

    let mut rng = rand::thread_rng();
    let bytes: [u8; 8] = {
        let mut buf = [0u8; 8];
        rng.fill_bytes(&mut buf);
        buf
    };
    let invite_code = hex::encode(bytes);

    let link = format!(
        "ohc://join?ref={}&utm_source=cloud_bridge&utm_medium=team_share&inviter={}&team={}",
        invite_code, inviter_id, team_id
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

    #[test]
    fn test_generate_team_invite_link() {
        let link = generate_team_invite_link("team123", "user123").unwrap();
        assert!(link.starts_with("ohc://join?ref="));
        assert!(link.contains("utm_source=cloud_bridge"));
        assert!(link.contains("utm_medium=team_share"));
        assert!(link.contains("inviter=user123"));
        assert!(link.contains("team=team123"));

        let err = generate_team_invite_link("", "user123").unwrap_err();
        assert_eq!(err, "team_id and inviter_id cannot be empty");

        let err2 = generate_team_invite_link("team123", "").unwrap_err();
        assert_eq!(err2, "team_id and inviter_id cannot be empty");
    }
}
