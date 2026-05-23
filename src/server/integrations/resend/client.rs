pub struct ResendClient {
    pub api_key: String,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        ResendClient { api_key }
    }
}
