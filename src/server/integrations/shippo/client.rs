pub struct ShippoClient {
    token: String,
}

impl ShippoClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
