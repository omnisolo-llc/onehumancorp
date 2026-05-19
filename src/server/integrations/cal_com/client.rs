pub struct CalComClient {
    pub access_token: String,
}

impl CalComClient {
    pub fn new(access_token: String) -> Self {
        CalComClient { access_token }
    }
}
