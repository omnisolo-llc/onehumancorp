use serde_json::Value;

pub struct WorkTriageService {}

impl WorkTriageService {
    pub fn calculate_priority_score(event_source: &str, context_payload: &Option<Value>) -> i32 {
        let mut score = 0;

        match event_source {
            "payment_failed" | "deposit_failed" | "urgent_alert" => score += 100,
            "inventory_alert" | "low_stock" => score += 80,
            "booking_request" => score += 50,
            "customer_message" | "instagram_dm" | "omnichannel_gateway" => score += 30,
            _ => score += 10,
        }

        // Further refine score based on payload hints
        if let Some(payload) = context_payload {
            if let Some(priority_str) = payload.get("priority").and_then(|v| v.as_str()) {
                if priority_str.eq_ignore_ascii_case("urgent") {
                    score += 50;
                } else if priority_str.eq_ignore_ascii_case("high") {
                    score += 20;
                }
            }
        }

        score
    }

    pub fn extract_correlation_id(event_source: &str, context_payload: &Option<Value>) -> Option<String> {
        if let Some(payload) = context_payload {
            // Check for explicit correlation_id first
            if let Some(cid) = payload.get("correlation_id").and_then(|v| v.as_str()) {
                return Some(cid.to_string());
            }

            // Fallback heuristics based on source
            match event_source {
                "inventory_alert" | "low_stock" => {
                    if let Some(sku) = payload.get("sku").and_then(|v| v.as_str()) {
                        return Some(format!("low_stock_{}", sku));
                    }
                },
                "customer_message" | "instagram_dm" | "omnichannel_gateway" => {
                     if let Some(cust_id) = payload.get("customer_id").and_then(|v| v.as_str()) {
                        return Some(format!("messages_{}", cust_id));
                    }
                },
                _ => {}
            }
        }
        None
    }
}
