pub struct MailchimpClient {
    token: String,
}

impl MailchimpClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
