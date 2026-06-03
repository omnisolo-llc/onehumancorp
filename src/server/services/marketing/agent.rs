use super::lob_client::LobClient;

pub struct MarketingAgent {
    lob_client: LobClient,
}

impl MarketingAgent {
    pub fn new(lob_client: LobClient) -> Self {
        Self { lob_client }
    }

    pub async fn handle_job_completed(&self, location: &str, _job_id: &str) -> Result<(), String> {
        let message = format!("Hi neighbor! We just completed a repair down the street near {}. Scan this QR code to book us!", location);

        match self.lob_client.dispatch_postcard(50, location, &message).await {
            Ok(receipt) => {
                println!("Marketing Agent success: {}", receipt);
                Ok(())
            }
            Err(e) => {
                println!("Marketing Agent failure: {}", e);
                Err(e)
            }
        }
    }
}
