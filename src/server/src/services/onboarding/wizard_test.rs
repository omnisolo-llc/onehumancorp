#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::onboarding::wizard::SetupWizardState;

    #[test]
    fn test_wizard_state_creation() {
        let state = SetupWizardState::new("tenant-123");
        assert_eq!(state.tenant_id, "tenant-123");
        assert_eq!(state.company_name, "");
    }
}
