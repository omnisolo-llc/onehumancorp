use crate::client::WalletPassClient;

pub struct WalletPassProvider {
    client: WalletPassClient,
}

impl WalletPassProvider {
    pub fn new() -> Self {
        Self {
            client: WalletPassClient::new(),
        }
    }

    pub fn client(&self) -> &WalletPassClient {
        &self.client
    }
}
