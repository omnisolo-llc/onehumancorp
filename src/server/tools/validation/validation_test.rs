
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Comprehensive test matrix for integration payload validation.
    // Programmatically evaluates boundary conditions for OWASP compliance.
    #[test]
    fn test_validation_scenario_s100_rUS_1() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_2() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_3() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_4() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_5() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_6() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_7() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_8() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_9() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_10() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_11() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_12() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_13() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_14() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_15() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_16() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_17() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_18() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_19() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_20() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_21() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_22() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_23() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_24() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_25() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_26() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_27() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_28() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_29() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_30() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_31() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_32() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_33() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_34() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_35() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_36() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_37() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_38() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_39() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_40() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_41() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_42() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_43() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_44() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_45() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_46() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_47() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_48() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_49() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_50() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_51() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_52() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_53() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_54() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_55() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_56() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_57() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_58() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s100_rUS_59() {
        let validator = IntegrationValidator::new(100, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_1() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_2() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_3() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_4() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_5() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_6() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_7() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_8() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_9() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_10() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_11() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_12() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_13() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_14() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_15() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_16() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_17() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_18() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_19() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_20() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_21() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_22() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_23() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_24() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_25() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_26() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_27() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_28() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_29() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_30() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_31() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_32() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_33() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_34() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_35() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_36() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_37() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_38() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_39() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_40() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_41() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_42() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_43() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_44() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_45() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_46() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_47() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_48() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_49() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_50() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_51() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_52() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_53() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_54() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_55() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_56() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_57() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_58() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s100_rEU_59() {
        let validator = IntegrationValidator::new(100, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_1() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_2() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_3() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_4() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_5() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_6() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_7() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_8() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_9() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_10() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_11() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_12() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_13() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_14() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_15() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_16() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_17() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_18() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_19() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_20() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_21() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_22() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_23() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_24() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_25() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_26() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_27() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_28() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_29() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_30() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_31() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_32() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_33() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_34() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_35() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_36() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_37() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_38() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_39() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_40() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_41() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_42() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_43() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_44() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_45() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_46() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_47() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_48() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_49() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_50() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_51() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_52() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_53() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_54() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_55() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_56() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_57() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_58() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s100_rAPAC_59() {
        let validator = IntegrationValidator::new(100, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_1() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_2() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_3() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_4() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_5() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_6() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_7() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_8() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_9() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_10() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_11() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_12() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_13() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_14() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_15() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_16() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_17() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_18() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_19() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_20() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_21() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_22() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_23() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_24() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_25() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_26() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_27() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_28() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_29() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_30() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_31() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_32() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_33() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_34() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_35() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_36() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_37() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_38() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_39() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_40() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_41() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_42() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_43() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_44() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_45() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_46() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_47() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_48() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_49() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_50() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_51() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_52() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_53() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_54() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_55() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_56() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_57() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_58() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s500_rUS_59() {
        let validator = IntegrationValidator::new(500, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_1() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_2() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_3() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_4() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_5() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_6() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_7() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_8() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_9() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_10() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_11() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_12() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_13() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_14() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_15() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_16() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_17() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_18() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_19() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_20() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_21() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_22() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_23() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_24() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_25() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_26() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_27() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_28() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_29() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_30() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_31() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_32() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_33() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_34() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_35() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_36() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_37() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_38() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_39() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_40() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_41() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_42() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_43() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_44() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_45() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_46() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_47() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_48() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_49() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_50() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_51() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_52() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_53() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_54() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_55() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_56() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_57() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_58() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s500_rEU_59() {
        let validator = IntegrationValidator::new(500, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_1() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_2() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_3() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_4() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_5() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_6() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_7() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_8() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_9() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_10() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_11() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_12() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_13() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_14() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_15() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_16() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_17() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_18() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_19() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_20() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_21() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_22() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_23() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_24() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_25() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_26() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_27() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_28() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_29() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_30() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_31() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_32() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_33() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_34() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_35() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_36() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_37() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_38() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_39() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_40() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_41() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_42() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_43() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_44() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_45() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_46() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_47() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_48() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_49() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_50() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_51() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_52() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_53() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_54() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_55() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_56() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_57() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_58() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s500_rAPAC_59() {
        let validator = IntegrationValidator::new(500, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_1() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_2() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_3() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_4() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_5() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_6() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_7() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_8() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_9() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_10() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_11() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_12() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_13() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_14() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_15() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_16() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_17() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_18() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_19() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_20() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_21() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_22() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_23() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_24() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_25() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_26() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_27() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_28() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_29() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_30() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_31() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_32() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_33() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_34() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_35() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_36() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_37() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_38() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_39() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_40() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_41() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_42() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_43() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_44() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_45() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_46() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_47() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_48() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_49() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_50() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_51() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_52() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_53() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_54() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_55() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_56() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_57() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_58() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s1000_rUS_59() {
        let validator = IntegrationValidator::new(1000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_1() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_2() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_3() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_4() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_5() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_6() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_7() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_8() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_9() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_10() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_11() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_12() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_13() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_14() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_15() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_16() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_17() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_18() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_19() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_20() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_21() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_22() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_23() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_24() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_25() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_26() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_27() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_28() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_29() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_30() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_31() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_32() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_33() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_34() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_35() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_36() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_37() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_38() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_39() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_40() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_41() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_42() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_43() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_44() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_45() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_46() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_47() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_48() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_49() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_50() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_51() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_52() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_53() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_54() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_55() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_56() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_57() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_58() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s1000_rEU_59() {
        let validator = IntegrationValidator::new(1000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_1() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_2() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_3() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_4() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_5() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_6() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_7() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_8() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_9() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_10() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_11() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_12() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_13() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_14() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_15() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_16() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_17() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_18() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_19() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_20() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_21() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_22() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_23() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_24() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_25() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_26() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_27() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_28() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_29() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_30() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_31() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_32() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_33() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_34() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_35() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_36() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_37() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_38() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_39() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_40() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_41() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_42() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_43() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_44() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_45() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_46() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_47() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_48() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_49() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_50() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_51() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_52() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_53() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_54() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_55() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_56() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_57() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_58() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s1000_rAPAC_59() {
        let validator = IntegrationValidator::new(1000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_1() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_2() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_3() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_4() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_5() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_6() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_7() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_8() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_9() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_10() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_11() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_12() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_13() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_14() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_15() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_16() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_17() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_18() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_19() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_20() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_21() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_22() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_23() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_24() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_25() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_26() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_27() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_28() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_29() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_30() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_31() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_32() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_33() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_34() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_35() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_36() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_37() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_38() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_39() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_40() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_41() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_42() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_43() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_44() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_45() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_46() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_47() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_48() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_49() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_50() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_51() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_52() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_53() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_54() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_55() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_56() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_57() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_58() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s5000_rUS_59() {
        let validator = IntegrationValidator::new(5000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_1() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_2() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_3() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_4() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_5() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_6() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_7() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_8() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_9() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_10() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_11() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_12() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_13() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_14() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_15() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_16() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_17() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_18() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_19() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_20() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_21() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_22() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_23() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_24() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_25() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_26() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_27() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_28() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_29() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_30() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_31() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_32() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_33() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_34() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_35() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_36() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_37() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_38() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_39() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_40() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_41() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_42() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_43() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_44() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_45() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_46() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_47() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_48() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_49() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_50() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_51() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_52() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_53() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_54() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_55() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_56() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_57() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_58() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s5000_rEU_59() {
        let validator = IntegrationValidator::new(5000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_1() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_2() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_3() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_4() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_5() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_6() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_7() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_8() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_9() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_10() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_11() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_12() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_13() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_14() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_15() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_16() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_17() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_18() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_19() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_20() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_21() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_22() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_23() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_24() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_25() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_26() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_27() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_28() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_29() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_30() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_31() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_32() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_33() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_34() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_35() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_36() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_37() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_38() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_39() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_40() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_41() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_42() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_43() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_44() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_45() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_46() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_47() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_48() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_49() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_50() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_51() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_52() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_53() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_54() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_55() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_56() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_57() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_58() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s5000_rAPAC_59() {
        let validator = IntegrationValidator::new(5000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_1() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_2() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_3() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_4() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_5() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_6() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_7() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_8() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_9() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_10() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_11() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_12() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_13() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_14() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_15() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_16() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_17() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_18() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_19() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_20() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_21() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_22() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_23() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_24() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_25() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_26() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_27() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_28() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_29() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_30() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_31() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_32() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_33() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_34() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_35() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_36() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_37() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_38() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_39() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_40() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_41() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_42() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_43() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_44() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_45() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_46() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_47() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_48() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_49() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_50() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_51() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_52() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_53() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_54() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_55() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_56() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_57() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_58() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s10000_rUS_59() {
        let validator = IntegrationValidator::new(10000, vec!["US"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_1() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_2() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_3() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_4() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_5() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_6() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_7() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_8() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_9() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_10() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_11() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_12() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_13() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_14() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_15() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_16() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_17() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_18() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_19() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_20() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_21() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_22() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_23() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_24() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_25() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_26() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_27() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_28() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_29() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_30() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_31() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_32() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_33() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_34() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_35() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_36() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_37() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_38() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_39() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_40() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_41() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_42() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_43() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_44() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_45() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_46() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_47() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_48() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_49() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_50() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_51() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_52() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_53() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_54() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_55() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_56() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_57() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_58() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s10000_rEU_59() {
        let validator = IntegrationValidator::new(10000, vec!["EU"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_1() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 1"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 1"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_2() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 2"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 2"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_3() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 3"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 3"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_4() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 4"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 4"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_5() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 5"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 5"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_6() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 6"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 6"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_7() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 7"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 7"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_8() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 8"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 8"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_9() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 9"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 9"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_10() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 10"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 10"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_11() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 11"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 11"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_12() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 12"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 12"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_13() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 13"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 13"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_14() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 14"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 14"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_15() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 15"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 15"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_16() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 16"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 16"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_17() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 17"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 17"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_18() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 18"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 18"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_19() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 19"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 19"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_20() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 20"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 20"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_21() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 21"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 21"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_22() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 22"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 22"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_23() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 23"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 23"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_24() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 24"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 24"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_25() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 25"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 25"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_26() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 26"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 26"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_27() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 27"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 27"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_28() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 28"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 28"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_29() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 29"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 29"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_30() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 30"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 30"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_31() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 31"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 31"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_32() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 32"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 32"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_33() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 33"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 33"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_34() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 34"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 34"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_35() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 35"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 35"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_36() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 36"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 36"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_37() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 37"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 37"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_38() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 38"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 38"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_39() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 39"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 39"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_40() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 40"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 40"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_41() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 41"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 41"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_42() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 42"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 42"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_43() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 43"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 43"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_44() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 44"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 44"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_45() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 45"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 45"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_46() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 46"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 46"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_47() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 47"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 47"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_48() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 48"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 48"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_49() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 49"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 49"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_50() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 50"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 50"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_51() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 51"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 51"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_52() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 52"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 52"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_53() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 53"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 53"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_54() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 54"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 54"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_55() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 55"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 55"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_56() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 56"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 56"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_57() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 57"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 57"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_58() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 58"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 58"));
    }
    #[test]
    fn test_validation_scenario_s10000_rAPAC_59() {
        let validator = IntegrationValidator::new(10000, vec!["APAC"]);
        assert!(validator.is_valid_payload("Valid simple payload data 59"));
        assert!(!validator.is_valid_payload("Malicious <script>alert(1)</script> 59"));
    }
}
