pub struct EasyPostClient {
    pub api_key: String,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        EasyPostClient { api_key }
    }
}
