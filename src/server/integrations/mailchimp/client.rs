pub struct MailchimpClient {
    pub api_key: String,
    pub server_prefix: String,
}

impl MailchimpClient {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        MailchimpClient { api_key, server_prefix }
    }

    pub async fn add_member_to_list(&self, _list_id: &str, _email: &str, tenant_id: &str) -> Result<(), String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mailchimp_add_member",
            0.15
        ).await;
        Ok(())
    }
}
