pub struct ManychatClient {
    pub access_token: String,
}

impl ManychatClient {
    pub fn new(access_token: String) -> Self {
        ManychatClient { access_token }
    }
}
