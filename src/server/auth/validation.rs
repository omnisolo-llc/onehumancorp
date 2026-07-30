const RESERVED_USERNAMES: &[&str] = &[
    "admin",
    "administrator",
    "api",
    "billing",
    "help",
    "login",
    "logout",
    "register",
    "root",
    "security",
    "settings",
    "support",
    "system",
];

const COMMON_PASSWORDS: &[&str] = &[
    "123456789012",
    "correcthorsebatterystaple",
    "letmeinletmein",
    "password1234",
    "qwertyuiop12",
];

const RESERVED_ORGANIZATIONS: &[&str] = &["admin", "api", "root", "system", "support"];

pub fn normalize_email(input: &str) -> Result<String, &'static str> {
    let email = input.trim();
    if email.is_empty() || email.len() > 254 || !email.is_ascii() {
        return Err("enter a valid email address");
    }
    if email
        .bytes()
        .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err("enter a valid email address");
    }

    let mut parts = email.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || local.is_empty()
        || local.len() > 64
        || domain.is_empty()
        || domain.len() > 253
        || local.starts_with('.')
        || local.ends_with('.')
        || local.contains("..")
        || domain.starts_with('-')
        || domain.ends_with('-')
        || domain.starts_with('.')
        || domain.ends_with('.')
        || domain.contains("..")
        || !domain.contains('.')
    {
        return Err("enter a valid email address");
    }

    let local_valid = local.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'.' | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'/'
                    | b'='
                    | b'?'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'{'
                    | b'|'
                    | b'}'
                    | b'~'
            )
    });
    let domain_valid = domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    });
    if !local_valid || !domain_valid {
        return Err("enter a valid email address");
    }

    Ok(format!(
        "{}@{}",
        local.to_ascii_lowercase(),
        domain.to_ascii_lowercase()
    ))
}

pub fn normalize_username(input: &str) -> Result<String, &'static str> {
    let username = input.trim().to_ascii_lowercase();
    if username.len() < 3 || username.len() > 32 {
        return Err("username must be between 3 and 32 characters");
    }
    let bytes = username.as_bytes();
    if !bytes[0].is_ascii_alphanumeric()
        || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || username.contains("..")
        || RESERVED_USERNAMES.contains(&username.as_str())
    {
        return Err("username is not available");
    }
    Ok(username)
}

pub fn normalize_organization(input: &str) -> Result<String, &'static str> {
    let organization = input.trim().to_ascii_lowercase();
    if organization.len() < 3 || organization.len() > 48 {
        return Err("workspace ID must be between 3 and 48 characters");
    }
    let bytes = organization.as_bytes();
    if !bytes[0].is_ascii_alphanumeric()
        || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        || organization.contains("--")
        || RESERVED_ORGANIZATIONS.contains(&organization.as_str())
    {
        return Err("workspace ID is not available");
    }
    Ok(organization)
}

pub fn validate_password(password: &str, username: &str, email: &str) -> Result<(), &'static str> {
    let character_count = password.chars().count();
    if character_count < 12 || character_count > 128 {
        return Err("password must be between 12 and 128 characters");
    }
    if password.chars().any(char::is_control) {
        return Err("password contains unsupported characters");
    }

    let normalized = password.to_lowercase();
    let email_local = email
        .split('@')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if COMMON_PASSWORDS.contains(&normalized.as_str())
        || normalized.contains(&username.to_ascii_lowercase())
        || (!email_local.is_empty() && normalized.contains(&email_local))
    {
        return Err("choose a less predictable password");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_is_bounded_canonical_and_strict() {
        assert_eq!(
            normalize_email(" Alice+Ops@Example.COM ").unwrap(),
            "alice+ops@example.com"
        );
        for invalid in [
            "",
            "a@b",
            "a@@example.com",
            ".a@example.com",
            "a..b@example.com",
            "a@-example.com",
            "a example@example.com",
        ] {
            assert!(normalize_email(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn username_is_canonical_and_rejects_reserved_or_ambiguous_values() {
        assert_eq!(normalize_username(" Alice.Ops ").unwrap(), "alice.ops");
        for invalid in [
            "ab",
            "admin",
            "-alice",
            "alice-",
            "alice..ops",
            "alice ops",
            "álîce",
        ] {
            assert!(normalize_username(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn password_supports_passphrases_but_rejects_personal_or_common_values() {
        assert!(
            validate_password("violet river cabin orbit", "alice", "alice@example.com").is_ok()
        );
        assert!(validate_password("short", "alice", "alice@example.com").is_err());
        assert!(validate_password("alice-has-a-password", "alice", "alice@example.com").is_err());
        assert!(
            validate_password("correcthorsebatterystaple", "alice", "alice@example.com").is_err()
        );
    }

    #[test]
    fn organization_is_memorable_canonical_and_safe() {
        assert_eq!(
            normalize_organization(" Alice-Shop ").unwrap(),
            "alice-shop"
        );
        for invalid in [
            "ab",
            "system",
            "-alice",
            "alice-",
            "alice--shop",
            "alice_shop",
            "álîce",
        ] {
            assert!(
                normalize_organization(invalid).is_err(),
                "accepted {invalid}"
            );
        }
    }
}
