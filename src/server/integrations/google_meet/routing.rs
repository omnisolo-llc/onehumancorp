use axum::{
    extract::Query,
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn google_oauth_callback(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(_code) = params.get("code") {
        return "Google OAuth Successful".to_string();
    }
    "Failed".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_google_oauth_callback() {
        let mut map = HashMap::new();
        map.insert("code".to_string(), "abc".to_string());
        let _res = google_oauth_callback(Query(map)).await;
    }
}
