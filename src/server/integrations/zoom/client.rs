pub struct ZoomClient {
    token: String,
}

impl ZoomClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
