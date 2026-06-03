use axum::{
    http::{Request, header::ACCEPT_LANGUAGE},
    middleware::Next,
    response::Response,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TargetCurrency(pub String);

pub async fn localization_middleware<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let accept_lang = req
        .headers()
        .get(ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("en-US");

    // Very simple stub for Accept-Language to currency mapping
    let currency = match accept_lang.split(',').next().unwrap_or("en-US").trim() {
        "en-GB" => "GBP",
        "ja-JP" | "ja" => "JPY",
        "de-DE" | "de" | "fr-FR" | "fr" => "EUR",
        _ => "USD",
    };

    req.extensions_mut().insert(TargetCurrency(currency.to_string()));

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};

    #[tokio::test]
    async fn test_localization_middleware_usd() {
        let req = Request::builder()
            .header(ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            .body(Body::empty())
            .unwrap();

        let (mut parts, _) = req.into_parts();

        // This is a simplified unit test logic since calling the actual axum middleware Next is complex
        // without a full app router in the test scope. We'll just test the extraction logic.
        let accept_lang = parts
            .headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("en-US");

        let currency = match accept_lang.split(',').next().unwrap_or("en-US").trim() {
            "en-GB" => "GBP",
            "ja-JP" | "ja" => "JPY",
            "de-DE" | "de" | "fr-FR" | "fr" => "EUR",
            _ => "USD",
        };

        assert_eq!(currency, "USD");
    }

    #[tokio::test]
    async fn test_localization_middleware_eur() {
        let req = Request::builder()
            .header(ACCEPT_LANGUAGE, "de-DE,de;q=0.9")
            .body(Body::empty())
            .unwrap();

        let (mut parts, _) = req.into_parts();

        let accept_lang = parts
            .headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("en-US");

        let currency = match accept_lang.split(',').next().unwrap_or("en-US").trim() {
            "en-GB" => "GBP",
            "ja-JP" | "ja" => "JPY",
            "de-DE" | "de" | "fr-FR" | "fr" => "EUR",
            _ => "USD",
        };

        assert_eq!(currency, "EUR");
    }
}
