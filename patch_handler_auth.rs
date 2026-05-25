<<<<<<< SEARCH
    let is_system = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = &h[7..];
            if token == "system_internal_token" {
                true
            } else {
                // If it's a real user token, validate roles
                match state.store.validate_token(token).await {
                    Ok(claims) => claims.roles.contains(&"system".to_string()),
                    Err(_) => false,
                }
            }
        }
        _ => false,
    };
=======
    let is_system = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = &h[7..];
            let system_token = std::env::var("OHC_SYSTEM_TOKEN").unwrap_or_else(|_| "".to_string());
            if !system_token.is_empty() && token == system_token {
                true
            } else {
                // If it's a real user token, validate roles
                match state.store.validate_token(token).await {
                    Ok(claims) => claims.roles.contains(&"system".to_string()),
                    Err(_) => false,
                }
            }
        }
        _ => false,
    };
>>>>>>> REPLACE
