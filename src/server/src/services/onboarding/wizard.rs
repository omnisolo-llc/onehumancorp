pub struct SetupWizardState {
    pub tenant_id: String,
    pub business_type: String,
    pub company_name: String,
    pub admin_email: String,
}

impl SetupWizardState {
    pub fn new(tenant_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            business_type: String::new(),
            company_name: String::new(),
            admin_email: String::new(),
        }
    }
}
