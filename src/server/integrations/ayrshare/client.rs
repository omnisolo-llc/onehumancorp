pub struct AyrshareClient {
    pub api_key: String,
}

impl AyrshareClient {
    pub fn new(api_key: String) -> Self {
        AyrshareClient { api_key }
    }
}
