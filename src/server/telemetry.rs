use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<regex::Regex> = OnceLock::new();
static SSN_REGEX: OnceLock<regex::Regex> = OnceLock::new();
static CREDIT_CARD_REGEX: OnceLock<regex::Regex> = OnceLock::new();

pub fn contains_pii(payload: &str) -> bool {
    let email_regex = EMAIL_REGEX.get_or_init(|| regex::Regex::new(r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}").unwrap());
    let ssn_regex = SSN_REGEX.get_or_init(|| regex::Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap());
    let credit_card_regex = CREDIT_CARD_REGEX.get_or_init(|| regex::Regex::new(r"\b(?:\d{4}[ -]?){3}\d{4}\b").unwrap());

    email_regex.is_match(payload) || ssn_regex.is_match(payload) || credit_card_regex.is_match(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pii_detection() {
        assert!(contains_pii("Contact me at user@example.com"));
        assert!(contains_pii("My SSN is 123-45-6789."));
        assert!(contains_pii("Here is my card: 1234 5678 1234 5678"));

        assert!(!contains_pii("This is a safe message without PII."));
        assert!(!contains_pii("The quick brown fox jumps over 123 dogs."));
    }
}
