//! Integer cents in the application; exact decimal dollars only on the JSON wire.
use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;

/// Preserve the existing inclusion of all supplied line items while checking
/// multiplication and addition before a provider request or database write.
pub fn checked_total_cents(
    items: impl IntoIterator<Item = (i64, i32)>,
) -> Result<i64, &'static str> {
    items
        .into_iter()
        .try_fold(0_i64, |total, (price, quantity)| {
            if price < 0 || quantity < 0 {
                return Err("Quote prices and quantities must be nonnegative");
            }
            price
                .checked_mul(i64::from(quantity))
                .and_then(|amount| total.checked_add(amount))
                .ok_or("Quote total exceeds the supported cent amount")
        })
}

pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    // RawValue keeps the original JSON token, including exponent notation and
    // integers beyond f64's exact range. Never deserialize money through Value/f64.
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    let text = raw.get();
    let amount = if let Some((base, _)) = text.split_once(['e', 'E']) {
        // from_scientific uses the rounding FromStr parser for its mantissa.
        // Validate exact representability first so sub-cent tails cannot vanish.
        Decimal::from_str_exact(base).map_err(serde::de::Error::custom)?;
        Decimal::from_scientific(text)
    } else {
        Decimal::from_str_exact(text)
    }
    .map_err(serde::de::Error::custom)?;
    if amount < Decimal::ZERO {
        return Err(serde::de::Error::custom("TaxJar money must be nonnegative"));
    }
    let cents = amount
        .checked_mul(Decimal::ONE_HUNDRED)
        .ok_or_else(|| serde::de::Error::custom("TaxJar money exceeds supported cents"))?;
    if !cents.fract().is_zero() {
        return Err(serde::de::Error::custom(
            "TaxJar money contains fractional cents",
        ));
    }
    cents
        .to_i64()
        .ok_or_else(|| serde::de::Error::custom("TaxJar money exceeds supported cents"))
}

pub(crate) fn serialize<S: Serializer>(cents: &i64, serializer: S) -> Result<S::Ok, S::Error> {
    if *cents < 0 {
        return Err(serde::ser::Error::custom(
            "TaxJar money must be nonnegative",
        ));
    }
    RawValue::from_string(Decimal::new(*cents, 2).to_string())
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}
