use crate::integrations::shippo::client::ShippoClient;

pub struct ShippoProvider {
    #[allow(dead_code)]
    client: ShippoClient,
}

impl ShippoProvider {
    pub fn new(api_token: String) -> Self {
        Self {
            client: ShippoClient::new(api_token),
        }
    }
}
