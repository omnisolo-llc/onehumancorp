#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::api::loyalty::{CreateProgramRequest, EarnPointsRequest, RedeemRewardRequest};

    #[tokio::test]
    async fn test_create_program_request_deserialization() {
        let json_data = json!({
            "name": "Gold Tier",
            "program_type": "TIERS",
            "config": {
                "threshold": 1000
            }
        });

        let req: CreateProgramRequest = serde_json::from_value(json_data).unwrap();
        assert_eq!(req.name, "Gold Tier");
        assert_eq!(req.program_type, "TIERS");
    }

    #[tokio::test]
    async fn test_earn_points_request_deserialization() {
        let json_data = json!({
            "points": 500,
            "description": "Purchase"
        });

        let req: EarnPointsRequest = serde_json::from_value(json_data).unwrap();
        assert_eq!(req.points, 500);
        assert_eq!(req.description.unwrap(), "Purchase");
    }

    #[tokio::test]
    async fn test_redeem_reward_request_deserialization() {
        let json_data = json!({
            "points": 100,
            "description": "Free Coffee"
        });

        let req: RedeemRewardRequest = serde_json::from_value(json_data).unwrap();
        assert_eq!(req.points, 100);
        assert_eq!(req.description.unwrap(), "Free Coffee");
    }
}
