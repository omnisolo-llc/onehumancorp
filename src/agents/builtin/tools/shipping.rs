use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};
use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ShippingRateProposalArgs {
    pub tenant_id: String,
    pub order_id: String,
    pub weight: f64,
    pub dimensions: String,
    pub delivery_deadline_days: Option<u32>,
}

pub struct ShippingRateProposalExecutor {
    pub pool: sqlx::PgPool,
}

impl ShippingRateProposalExecutor {
    pub fn select_best_rate(
        rates: Vec<server_integrations_shippo::client::ShippoRate>,
        deadline: Option<u32>,
    ) -> Result<(server_integrations_shippo::client::ShippoRate, Vec<serde_json::Value>), String> {
        if rates.is_empty() {
            return Err("No shipping rates returned by the carrier.".to_string());
        }

        let mut qualifying_rates = vec![];
        let mut rejected_alternatives = vec![];

        for rate in rates {
            let amount: f64 = rate.amount.parse().unwrap_or(0.0);
            if let Some(dl) = deadline {
                if rate.days > 0 && rate.days <= dl {
                    qualifying_rates.push((rate.clone(), amount));
                } else {
                    rejected_alternatives.push(json!({
                        "id": rate.id,
                        "carrier": rate.carrier,
                        "service": rate.service,
                        "amount": rate.amount,
                        "days": rate.days,
                        "reason": format!("Delivery estimated in {} days, exceeds deadline of {} days", rate.days, dl)
                    }));
                }
            } else {
                qualifying_rates.push((rate.clone(), amount));
            }
        }

        if qualifying_rates.is_empty() {
            return Err(format!("No qualifying rates met the deadline of {} days.", deadline.unwrap_or(0)));
        }

        // Sort by amount (lowest first). If amounts are equal, sort by days (fastest first).
        qualifying_rates.sort_by(|a, b| {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.days.cmp(&b.0.days))
        });

        Ok((qualifying_rates[0].0.clone(), rejected_alternatives))
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippingRateProposalArgs> for ShippingRateProposalExecutor {
    async fn execute_typed(&self, args: ShippingRateProposalArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
        let order_id = args.order_id;
        let weight = args.weight;
        let dimensions = args.dimensions;
        let deadline = args.delivery_deadline_days;

        let token = std::env::var("SHIPPO_API_TOKEN").unwrap_or_default();
        if token.is_empty() {
            return Err(ToolError::LlmRecoverable("Shippo API token not configured".to_string()));
        }

        let client = server_integrations_shippo::client::ShippoClient::new(token.clone());
        let rates = match client.fetch_rates(weight, &dimensions).await {
            Ok(rates) => rates,
            Err(e) => return Err(ToolError::LlmRecoverable(format!("Failed to fetch rates: {}", e))),
        };

        let (selected_rate, rejected_alternatives) = match Self::select_best_rate(rates, deadline) {
            Ok(result) => result,
            Err(e) => return Ok(json!({
                "status": "error",
                "message": e,
            }).to_string())
        };

        // Persist draft proposal (Triage Item)
        let triage_id = Uuid::new_v4().to_string();
        let context = json!({
            "order_id": order_id,
            "weight": weight,
            "dimensions": dimensions,
            "proposed_rate": {
                "id": selected_rate.id,
                "carrier": selected_rate.carrier,
                "service": selected_rate.service,
                "amount": selected_rate.amount,
                "days": selected_rate.days,
            }
        });

        sqlx::query(
            "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Operations Agent', 'high', $3, 'pending')"
        )
        .bind(&triage_id)
        .bind(&tenant_id)
        .bind(context)
        .execute(&self.pool)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("Failed to insert triage item: {}", e)))?;

        Ok(json!({
            "status": "success",
            "message": "Draft label proposal created for human review.",
            "selected_rate": {
                "id": selected_rate.id,
                "carrier": selected_rate.carrier,
                "service": selected_rate.service,
                "amount": selected_rate.amount,
                "days": selected_rate.days
            },
            "rejected_alternatives": rejected_alternatives,
            "estimated_delivery_days": selected_rate.days,
            "total_cost": selected_rate.amount
        }).to_string())
    }
}

pub fn shipping_rate_proposal_tool(pool: sqlx::PgPool) -> Tool {
    Tool {
        name: "shipping_rate_proposal".to_string(),
        description: "Draft a shipping label proposal for an order. Fetches rates, filters by optional delivery deadline, and selects the lowest cost rate for human review.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "order_id": { "type": "string" },
                "weight": { "type": "number", "description": "Package weight in ounces" },
                "dimensions": { "type": "string", "description": "Package dimensions e.g. '10x8x6'" },
                "delivery_deadline_days": { "type": "integer", "description": "Optional maximum number of days for delivery" }
            },
            "required": ["tenant_id", "order_id", "weight", "dimensions"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippingRateProposalExecutor { pool })),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use server_integrations_shippo::client::ShippoRate;

    #[test]
    pub fn test_rate_selection_logic_tie_breaker_and_sort() {
        let rates = vec![
            ShippoRate { id: "1".to_string(), carrier: "UPS".to_string(), service: "Ground".to_string(), amount: "10.00".to_string(), days: 5 },
            ShippoRate { id: "2".to_string(), carrier: "USPS".to_string(), service: "Priority".to_string(), amount: "15.00".to_string(), days: 2 },
            ShippoRate { id: "3".to_string(), carrier: "FedEx".to_string(), service: "Overnight".to_string(), amount: "30.00".to_string(), days: 1 },
            ShippoRate { id: "4".to_string(), carrier: "UPS".to_string(), service: "SurePost".to_string(), amount: "10.00".to_string(), days: 7 },
        ];

        let (selected, rejected) = ShippingRateProposalExecutor::select_best_rate(rates.clone(), None).unwrap();
        assert_eq!(selected.id, "1"); // 10.00, 5 days
        assert_eq!(rejected.len(), 0);

        let (selected, rejected) = ShippingRateProposalExecutor::select_best_rate(rates.clone(), Some(3)).unwrap();
        assert_eq!(selected.id, "2"); // 15.00, 2 days
        assert_eq!(rejected.len(), 2);
    }

    #[test]
    pub fn test_rate_selection_no_qualifying() {
        let rates = vec![
            ShippoRate { id: "1".to_string(), carrier: "UPS".to_string(), service: "Ground".to_string(), amount: "10.00".to_string(), days: 5 },
            ShippoRate { id: "4".to_string(), carrier: "UPS".to_string(), service: "SurePost".to_string(), amount: "10.00".to_string(), days: 7 },
        ];

        let result = ShippingRateProposalExecutor::select_best_rate(rates, Some(3));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No qualifying rates met the deadline of 3 days.");
    }
}
