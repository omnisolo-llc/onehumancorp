pub struct ManychatClient {
    pub api_key: String,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        ManychatClient { api_key }
    }
}
