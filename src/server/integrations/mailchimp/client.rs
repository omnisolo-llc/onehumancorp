pub struct MailchimpClient {
    pub api_key: String,
}

impl MailchimpClient {
    pub fn new(api_key: String) -> Self {
        MailchimpClient { api_key }
    }

    pub async fn sync_customer(&self, email: &str, _tag: &str) -> Result<(), String> {
        // Mock sync customer
        let pool = crate::db::get_pool();
        let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("mock_tenant")
            .bind("Mock Customer")
            .bind(email)
            .bind("mock_phone")
            .execute(&pool)
            .await;
        Ok(())
    }

    pub async fn send_campaign(&self, _audience: &str, _body: &str) -> Result<(), String> {
        // Mock send campaign
        Ok(())
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        Ok(())
    }
}
