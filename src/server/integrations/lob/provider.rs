use crate::integrations::lob::client::{LobClient, PostcardRequest, Address};

pub struct LobProvider {
    client: LobClient,
}

impl LobProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: LobClient::new(api_key),
        }
    }

    pub async fn send_postcard(&self, description: &str, name: &str, address_line1: &str, city: &str, state: &str, zip: &str, front_html: &str, back_html: &str) -> Result<String, String> {
        let req = PostcardRequest {
            description: description.to_string(),
            to: Address {
                name: name.to_string(),
                address_line1: address_line1.to_string(),
                address_line2: None,
                address_city: city.to_string(),
                address_state: state.to_string(),
                address_zip: zip.to_string(),
                address_country: "US".to_string(),
            },
            front: front_html.to_string(),
            back: back_html.to_string(),
        };

        match self.client.create_postcard(&req).await {
            Ok(res) => Ok(res.id),
            Err(e) => Err(e),
        }
    }
}
