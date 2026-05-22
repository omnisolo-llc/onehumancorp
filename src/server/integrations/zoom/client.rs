pub struct ZoomClient {
    pub api_key: String,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient { api_key }
    }
}
