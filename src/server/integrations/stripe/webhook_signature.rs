//! Stripe v1 authentication over the untouched request bytes.
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_BODY_BYTES: usize = 1024 * 1024;
pub const MAX_HEADER_BYTES: usize = 4096;
const MAX_SIGNATURES: usize = 16;
const TOLERANCE_SECONDS: i64 = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureError {
    MissingSecret,
    BodyTooLarge,
    InvalidHeader,
    InvalidTimestamp,
    InvalidSignature,
}
impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MissingSecret => "webhook signing secret unavailable",
            Self::BodyTooLarge => "webhook body too large",
            Self::InvalidHeader => "invalid webhook signature header",
            Self::InvalidTimestamp => "webhook timestamp outside tolerance",
            Self::InvalidSignature => "invalid webhook signature",
        })
    }
}
impl std::error::Error for SignatureError {}

pub fn verify_at(
    payload: &[u8],
    header: &str,
    secret: &[u8],
    now: i64,
) -> Result<(), SignatureError> {
    if secret.is_empty() || secret.len() > 4096 || secret.iter().all(u8::is_ascii_whitespace) {
        return Err(SignatureError::MissingSecret);
    }
    if payload.len() > MAX_BODY_BYTES {
        return Err(SignatureError::BodyTooLarge);
    }
    if header.is_empty() || header.len() > MAX_HEADER_BYTES {
        return Err(SignatureError::InvalidHeader);
    }
    let mut timestamp = None;
    let mut signatures = Vec::new();
    for field in header.split(',') {
        let (name, value) = field
            .trim()
            .split_once('=')
            .ok_or(SignatureError::InvalidHeader)?;
        let value = value.trim();
        match name.trim() {
            "t" => {
                if timestamp.is_some()
                    || value.is_empty()
                    || !value.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(SignatureError::InvalidHeader);
                }
                let number = value
                    .parse::<i64>()
                    .map_err(|_| SignatureError::InvalidHeader)?;
                timestamp = Some((value, number));
            }
            "v1" => {
                if signatures.len() >= MAX_SIGNATURES || value.len() != 64 {
                    return Err(SignatureError::InvalidHeader);
                }
                signatures.push(hex::decode(value).map_err(|_| SignatureError::InvalidHeader)?);
            }
            _ => {} // Unknown versions cannot authorize a request.
        }
    }
    let (timestamp_text, timestamp) = timestamp.ok_or(SignatureError::InvalidHeader)?;
    if signatures.is_empty() {
        return Err(SignatureError::InvalidHeader);
    }
    let skew = now
        .checked_sub(timestamp)
        .and_then(i64::checked_abs)
        .ok_or(SignatureError::InvalidTimestamp)?;
    if skew > TOLERANCE_SECONDS {
        return Err(SignatureError::InvalidTimestamp);
    }
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret).map_err(|_| SignatureError::MissingSecret)?;
    mac.update(timestamp_text.as_bytes());
    mac.update(b".");
    mac.update(payload);
    let mut matched = false;
    // Mac::verify_slice is constant-time. Check all bounded v1 candidates so
    // endpoint secret rotation does not depend on the signature's position.
    for signature in signatures {
        matched |= mac.clone().verify_slice(&signature).is_ok();
    }
    if matched {
        Ok(())
    } else {
        Err(SignatureError::InvalidSignature)
    }
}

pub fn verify_now(payload: &[u8], header: &str, secret: &[u8]) -> Result<(), SignatureError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|time| i64::try_from(time.as_secs()).ok())
        .ok_or(SignatureError::InvalidTimestamp)?;
    verify_at(payload, header, secret, now)
}

#[cfg(test)]
mod tests {
    use super::*;
    const NOW: i64 = 1_700_000_000;
    const BODY: &[u8] =
        b"{\"id\":\"evt_fixture\",\"type\":\"test.event\",\"data\":{\"object\":{}}}\n";
    const SECRET: &[u8] = b"whsec_public_test_only";
    // Independently generated with Python's standard-library hmac/hashlib.
    const DIGEST: &str = "0956895e98502ae28bebda093b3887a791844e0441a951aa9b6f58a5fdafa514";
    fn header() -> String {
        format!("t={NOW},v1={DIGEST}")
    }
    #[test]
    fn independent_vector_authenticates_only_the_original_bytes() {
        assert!(verify_at(BODY, &header(), SECRET, NOW).is_ok());
        assert!(verify_at(&BODY[..BODY.len() - 1], &header(), SECRET, NOW).is_err());
        let changed = String::from_utf8(BODY.to_vec())
            .unwrap()
            .replace("test.event", "changed.event");
        assert!(verify_at(changed.as_bytes(), &header(), SECRET, NOW).is_err());
        let reformatted =
            serde_json::to_vec_pretty(&serde_json::from_slice::<serde_json::Value>(BODY).unwrap())
                .unwrap();
        assert!(verify_at(&reformatted, &header(), SECRET, NOW).is_err());
        assert!(verify_at(BODY, &header(), b"different endpoint", NOW).is_err());
    }
    #[test]
    fn rotation_checks_all_v1_values_and_never_authorizes_v0() {
        assert!(
            verify_at(
                BODY,
                &format!(
                    "t={NOW},v1={},v0={},v1={DIGEST}",
                    "0".repeat(64),
                    "0".repeat(64)
                ),
                SECRET,
                NOW
            )
            .is_ok()
        );
        assert!(verify_at(BODY, &format!("t={NOW},v0={DIGEST}"), SECRET, NOW).is_err());
    }
    #[test]
    fn timestamp_boundaries_include_bounded_future_skew_without_overflow() {
        for offset in [-300, 0, 300] {
            assert!(verify_at(BODY, &header(), SECRET, NOW + offset).is_ok());
        }
        for time in [NOW - 301, NOW + 301, i64::MIN, i64::MAX] {
            assert!(verify_at(BODY, &header(), SECRET, time).is_err());
        }
    }
    #[test]
    fn absent_secret_and_ambiguous_or_malformed_headers_fail_closed() {
        for secret in [&b""[..], &b" \n\t"[..]] {
            assert_eq!(
                verify_at(BODY, &header(), secret, NOW),
                Err(SignatureError::MissingSecret)
            );
        }
        for value in [
            String::new(),
            format!("t={NOW},t={NOW},v1={DIGEST}"),
            format!("t=+{NOW},v1={DIGEST}"),
            format!("t=999999999999999999999,v1={DIGEST}"),
            format!("t={NOW},v1=nothex"),
            format!("v1={DIGEST}"),
            format!("t={NOW}"),
            format!("t={NOW},v1={},", DIGEST),
            format!("t={NOW},v1={}", "g".repeat(64)),
        ] {
            assert!(verify_at(BODY, &value, SECRET, NOW).is_err(), "{value}");
        }
    }
    #[test]
    fn body_header_and_signature_count_are_bounded() {
        assert_eq!(
            verify_at(&vec![0; MAX_BODY_BYTES + 1], &header(), SECRET, NOW),
            Err(SignatureError::BodyTooLarge)
        );
        assert_eq!(
            verify_at(BODY, &"x".repeat(MAX_HEADER_BYTES + 1), SECRET, NOW),
            Err(SignatureError::InvalidHeader)
        );
        let value = format!("t={NOW},{}", vec![format!("v1={DIGEST}"); 17].join(","));
        assert_eq!(
            verify_at(BODY, &value, SECRET, NOW),
            Err(SignatureError::InvalidHeader)
        );
    }
}
