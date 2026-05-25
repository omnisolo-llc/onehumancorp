<<<<<<< SEARCH
            match self.client.post(&url)
                // For a system-level role we can pass a dummy/service token or rely on a standard header
                // However, `RequireRole` logic normally parses authorization header
                // I will add a default bearer system if we haven't got a specific one
                .header("Authorization", "Bearer system_internal_token")
                .json(&req_payload)
                .send()
                .await
=======
            let system_token = std::env::var("OHC_SYSTEM_TOKEN").unwrap_or_else(|_| "missing_token".to_string());
            match self.client.post(&url)
                .header("Authorization", format!("Bearer {}", system_token))
                .json(&req_payload)
                .send()
                .await
>>>>>>> REPLACE
