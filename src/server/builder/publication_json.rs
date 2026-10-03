//! Bounded publication JSON admission before structural deserialization.
use super::publication_store::PublicationError;
use serde::Deserializer;
use serde::de::{self, MapAccess, Visitor};
use serde_json::{Map, Value, value::RawValue};
use std::fmt;

pub const MAX_PUBLICATION_BODY_BYTES: usize = 2 * 1024 * 1024;
const MAX_DEPTH: usize = 32;

fn invalid() -> PublicationError {
    PublicationError::Invalid("Invalid publication JSON")
}

/// Compare the original decimal literal before IEEE754 rounding can bring an
/// out-of-range value back inside the accepted safe-integer magnitude.
fn numeric_literal_is_safe(raw: &str) -> bool {
    let unsigned = raw.strip_prefix('-').unwrap_or(raw);
    let (coefficient, exponent) = unsigned.split_once(['e', 'E']).unwrap_or((unsigned, "0"));
    let negative_exponent = exponent.starts_with('-');
    let exponent = exponent.trim_start_matches(['+', '-']);
    let exponent = exponent.bytes().fold(0_i64, |n, digit| {
        n.saturating_mul(10).saturating_add(i64::from(digit - b'0'))
    });
    let exponent = if negative_exponent {
        -exponent
    } else {
        exponent
    };
    let fraction_digits = coefficient
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let digits: String = coefficient.chars().filter(|c| *c != '.').collect();
    let significant = digits.trim_start_matches('0');
    if significant.is_empty() {
        return true;
    }
    let position = (significant.len() as i64)
        .saturating_add(exponent)
        .saturating_sub(fraction_digits as i64);
    if position > 16 {
        return false;
    }
    if position == 16 {
        let mut whole = significant.chars().take(16).collect::<String>();
        while whole.len() < 16 {
            whole.push('0');
        }
        match whole.as_str().cmp("9007199254740991") {
            std::cmp::Ordering::Greater => return false,
            std::cmp::Ordering::Equal
                if significant.bytes().skip(16).any(|digit| digit != b'0') =>
            {
                return false;
            }
            _ => {}
        }
    }
    raw.parse::<f64>()
        .is_ok_and(|number| number.is_finite() && number != 0.0)
}

struct ObjectVisitor {
    depth: usize,
}
impl<'de> Visitor<'de> for ObjectVisitor {
    type Value = Map<String, Value>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a publication object without duplicate keys")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut object = Map::new();
        while let Some(key) = access.next_key::<String>()? {
            if key.contains('\0') || object.contains_key(&key) {
                return Err(de::Error::custom("Invalid or duplicate publication key"));
            }
            let raw = access.next_value::<Box<RawValue>>()?;
            let value = decode_raw(&raw, self.depth + 1)
                .map_err(|_| de::Error::custom("Invalid nested publication value"))?;
            object.insert(key, value);
        }
        Ok(object)
    }
}

fn decode_raw(raw: &RawValue, depth: usize) -> Result<Value, PublicationError> {
    let text = raw.get();
    match text.as_bytes().first() {
        Some(b'{') => {
            if depth >= MAX_DEPTH {
                return Err(invalid());
            }
            let mut deserializer = serde_json::Deserializer::from_str(text);
            let object = deserializer
                .deserialize_map(ObjectVisitor { depth })
                .map_err(|_| invalid())?;
            deserializer.end().map_err(|_| invalid())?;
            Ok(Value::Object(object))
        }
        Some(b'[') => {
            if depth >= MAX_DEPTH {
                return Err(invalid());
            }
            let values: Vec<Box<RawValue>> = serde_json::from_str(text).map_err(|_| invalid())?;
            values
                .iter()
                .map(|value| decode_raw(value, depth + 1))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array)
        }
        Some(b'"') => {
            let value: String = serde_json::from_str(text).map_err(|_| invalid())?;
            if value.contains('\0') {
                return Err(invalid());
            }
            Ok(Value::String(value))
        }
        Some(b'-' | b'0'..=b'9') if !numeric_literal_is_safe(text) => Err(invalid()),
        _ => serde_json::from_str(text).map_err(|_| invalid()),
    }
}

pub fn decode_publication_json(bytes: &[u8]) -> Result<Value, PublicationError> {
    if bytes.len() > MAX_PUBLICATION_BODY_BYTES {
        return Err(invalid());
    }
    let raw: Box<RawValue> = serde_json::from_slice(bytes).map_err(|_| invalid())?;
    decode_raw(&raw, 0)
}
