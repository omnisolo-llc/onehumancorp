use crate::miser::MiserRecommendation;

/// Protobuf-compatible action structure for the frontend.
/// This matches the hub.proto MiserAction message.
pub struct MiserAction {
    pub id: String,
    pub title: String,
    pub impact: String,
    pub potential_savings_cents: i64,
    pub action_type: String,
}

/// Converts internal domain recommendations into UI-ready actions.
pub fn adapt_recommendation(r: MiserRecommendation) -> MiserAction {
    MiserAction {
        id: r.id,
        title: r.title,
        impact: r.impact,
        potential_savings_cents: r.potential_savings_cents,
        action_type: r.action_type,
    }
}

/// Batch conversion utility for lists of recommendations.
pub fn adapt_recommendations(recs: Vec<MiserRecommendation>) -> Vec<MiserAction> {
    recs.into_iter().map(adapt_recommendation).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::miser::MiserRecommendation;

    #[test]
    fn test_adapter_consistency() {
        let r = MiserRecommendation {
            id: "ach_opt".to_string(),
            title: "Switch to ACH".to_string(),
            description: "Save fees by using bank transfers.".to_string(),
            impact: "Save $5".to_string(),
            action_label: "Connect Bank".to_string(),
            action_type: "PAYMENT".to_string(),
            potential_savings_cents: 500,
            priority: 1,
        };

        let adapted = adapt_recommendation(r);
        assert_eq!(adapted.id, "ach_opt");
        assert_eq!(adapted.potential_savings_cents, 500);
    }

    #[test]
    fn test_batch_adapter() {
        let recs = vec![
            MiserRecommendation {
                id: "1".to_string(),
                title: "T1".to_string(),
                description: "D1".to_string(),
                impact: "I1".to_string(),
                action_label: "A1".to_string(),
                action_type: "T1".to_string(),
                potential_savings_cents: 10,
                priority: 1,
            }
        ];
        let adapted = adapt_recommendations(recs);
        assert_eq!(adapted.len(), 1);
    }
}
