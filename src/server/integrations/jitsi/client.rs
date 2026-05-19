pub struct JitsiClient {
    pub api_key: String,
}

impl JitsiClient {
    pub fn new(api_key: String) -> Self {
        JitsiClient { api_key }
    }
}
