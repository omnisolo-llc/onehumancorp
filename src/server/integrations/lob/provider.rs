use crate::integrations::lob::client::{LobClient, PostcardRequest};

pub struct LobProvider {
    client: LobClient,
}

impl LobProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: LobClient::new(api_key),
        }
    }

    pub async fn send_postcard(&self, request: &PostcardRequest) -> Result<String, String> {
        match self.client.create_postcard(request).await {
            Ok(res) => Ok(res.id),
            Err(e) => Err(e),
        }
    }
}
