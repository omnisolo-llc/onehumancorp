use axum::{
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    pub currency: String,
    pub country: String,
}

pub async fn detect_currency(
    headers: HeaderMap,
) -> Result<Json<GeoLocation>, String> {
    let mut country = "US".to_string();

    if let Some(cf_ipcountry) = headers.get("CF-IPCountry") {
        if let Ok(c) = cf_ipcountry.to_str() {
            country = c.to_string();
        }
    } else if let Some(x_country) = headers.get("X-Vercel-IP-Country") {
        if let Ok(c) = x_country.to_str() {
            country = c.to_string();
        }
    }

    let currency = match country.as_str() {
        "US" => "USD".to_string(),
        "CA" => "CAD".to_string(),
        "GB" | "UK" => "GBP".to_string(),
        "DE" | "FR" | "IT" | "ES" | "NL" => "EUR".to_string(),
        "IN" => "INR".to_string(),
        "BR" => "BRL".to_string(),
        "MX" => "MXN".to_string(),
        "AU" => "AUD".to_string(),
        "JP" => "JPY".to_string(),
        _ => "USD".to_string(),
    };

    Ok(Json(GeoLocation { currency, country }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[tokio::test]
    async fn test_detect_currency_default() {
        let headers = HeaderMap::new();
        let res = detect_currency(headers).await.unwrap();
        assert_eq!(res.currency, "USD");
        assert_eq!(res.country, "US");
    }

    #[tokio::test]
    async fn test_detect_currency_cf() {
        let mut headers = HeaderMap::new();
        headers.insert("CF-IPCountry", HeaderValue::from_static("DE"));
        let res = detect_currency(headers).await.unwrap();
        assert_eq!(res.currency, "EUR");
        assert_eq!(res.country, "DE");
    }

    #[tokio::test]
    async fn test_detect_currency_vercel() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Vercel-IP-Country", HeaderValue::from_static("IN"));
        let res = detect_currency(headers).await.unwrap();
        assert_eq!(res.currency, "INR");
        assert_eq!(res.country, "IN");
    }
}
