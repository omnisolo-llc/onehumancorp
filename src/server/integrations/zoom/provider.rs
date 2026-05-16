use crate::integrations::zoom::client::ZoomClient;

pub struct ZoomProvider {
    #[allow(dead_code)]
    client: ZoomClient,
}

impl ZoomProvider {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: ZoomClient::new(client_id, client_secret),
        }
    }
}
