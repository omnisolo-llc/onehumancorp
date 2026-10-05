use std::collections::HashMap;

/// Decode a webhook form with the same WHATWG parser used for its signature.
/// Repeated decoded keys keep their last value, as the existing handlers did.
/// Bare fields become empty values; malformed escapes remain literal and invalid
/// UTF-8 is replaced with U+FFFD according to the form-url-encoded standard.
pub fn parse_form_urlencoded(input: &[u8]) -> HashMap<String, String> {
    url::form_urlencoded::parse(input).into_owned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encoded_unicode_is_decoded_as_utf8() {
        let params = parse_form_urlencoded(b"Body=caf%C3%A9+%F0%9F%98%80");
        assert_eq!(params["Body"], "café 😀");
    }

    #[test]
    fn values_preserve_encoded_and_literal_delimiters() {
        let params = parse_form_urlencoded(b"Body=a=b%3Dc%26d&Plus=+%2B&From=whatsapp%3A%2B123");
        assert_eq!(params["Body"], "a=b=c&d");
        assert_eq!(params["Plus"], " +");
        assert_eq!(params["From"], "whatsapp:+123");
    }

    #[test]
    fn duplicate_decoded_keys_keep_the_last_value_in_wire_order() {
        let params = parse_form_urlencoded(b"Body=first&%42ody=last&Other=one&Body=final");
        assert_eq!(params["Body"], "final");
        assert_eq!(params["Other"], "one");
    }

    #[test]
    fn empty_and_malformed_fields_follow_the_signature_parser() {
        let params = parse_form_urlencoded(b"Empty=&Bare&=value&&Broken=%Q1%&Bad=%FF&Raw=\xff");
        assert_eq!(params.len(), 6);
        assert_eq!(params["Empty"], "");
        assert_eq!(params["Bare"], "");
        assert_eq!(params[""], "value");
        assert_eq!(params["Broken"], "%Q1%");
        assert_eq!(params["Bad"], "�");
        assert_eq!(params["Raw"], "�");
        assert!(parse_form_urlencoded(b"&&").is_empty());
    }
}
