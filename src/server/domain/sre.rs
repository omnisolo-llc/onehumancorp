

use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub severity: String, // P0, P1, P2
    pub summary: String,
    pub root_cause_analysis: String,
    pub resolution_plan_id: String,
    pub status: String, // INVESTIGATING, PROPOSED, RESOLVED
}

pub struct AlertParser;

impl AlertParser {
    pub fn parse_prometheus_alert(&self, alert: &str) -> Result<Incident, String> {
        if alert.contains("HighErrorRate") {
            let id = format!("inc-{}", Utc::now().format("%Y%m%d%H%M%S"));
            return Ok(Incident {
                id,
                severity: "P0".to_string(),
                summary: "HighErrorRate".to_string(),
                root_cause_analysis: "".to_string(),
                resolution_plan_id: "".to_string(),
                status: "INVESTIGATING".to_string(),
            });
        }
        Err("unknown alert".to_string())
    }
}

pub struct RCAEngine;

impl RCAEngine {
    pub fn evaluate_confidence(&self, confidence: f64) -> String {
        if confidence < 0.85 { // Stricter confidence threshold for AUTO_REPAIR
            "WARM_HANDOFF".to_string()
        } else {
            "AUTO_REPAIR".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_parser_parse_prometheus_alert() {
        let parser = AlertParser;

        // Valid Alert HighErrorRate
        let alert = "firing: HighErrorRate in billing-engine";
        let res = parser.parse_prometheus_alert(alert);
        assert!(res.is_ok());
        let incident = res.unwrap();
        assert_eq!(incident.summary, "HighErrorRate");
        assert_eq!(incident.severity, "P0");
        assert_eq!(incident.status, "INVESTIGATING");
        assert!(incident.id.starts_with("inc-"));

        // Unknown Alert
        let alert = "firing: UnknownAlert in billing-engine";
        let res = parser.parse_prometheus_alert(alert);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "unknown alert");
    }

    #[test]
    fn test_rca_engine_evaluate_confidence() {
        let engine = RCAEngine;

        let tests = vec![
            ("Low confidence triggers warm handoff", 0.79, "WARM_HANDOFF"),
            ("Borderline confidence triggers warm handoff", 0.84, "WARM_HANDOFF"),
            ("Very low confidence triggers warm handoff", 0.50, "WARM_HANDOFF"),
            ("Exact threshold allows auto repair", 0.85, "AUTO_REPAIR"),
            ("High confidence allows auto repair", 0.95, "AUTO_REPAIR"),
        ];

        for (name, confidence, want) in tests {
            let got = engine.evaluate_confidence(confidence);
            assert_eq!(got, want, "Failed on test case: {}", name);
        }
    }
}
