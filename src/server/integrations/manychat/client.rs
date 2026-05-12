pub struct ManychatClient {
    token: String,
}

impl ManychatClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
