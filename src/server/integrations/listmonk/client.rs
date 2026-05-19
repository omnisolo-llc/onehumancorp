pub struct ListmonkClient {
    pub api_key: String,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        ListmonkClient { api_key }
    }
}
