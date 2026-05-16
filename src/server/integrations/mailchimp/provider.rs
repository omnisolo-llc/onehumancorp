use crate::integrations::mailchimp::client::MailchimpClient;

pub struct MailchimpProvider {
    #[allow(dead_code)]
    client: MailchimpClient,
}

impl MailchimpProvider {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        Self {
            client: MailchimpClient::new(api_key, server_prefix),
        }
    }
}
