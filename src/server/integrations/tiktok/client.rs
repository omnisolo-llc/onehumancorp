pub struct TikTokClient {
    _access_token: String,
}

impl TikTokClient {
    pub fn new(access_token: String) -> Self {
        Self {
            _access_token: access_token,
        }
    }
}
