use super::client::{ShippoClient, ShipmentRequest, TransactionRequest, RefundRequest};

pub struct ShippoProvider {
    client: ShippoClient,
}

impl ShippoProvider {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: ShippoClient::new(api_key),
        }
    }

    pub async fn get_rates(&self, req: &ShipmentRequest) -> Result<Vec<super::client::Rate>, super::client::ShippoError> {
        let resp = self.client.create_shipment(req).await?;
        Ok(resp.rates)
    }

    pub async fn print_label(&self, rate_id: &str) -> Result<super::client::TransactionResponse, super::client::ShippoError> {
        let req = TransactionRequest {
            rate: rate_id.to_string(),
            label_file_type: "PDF".to_string(),
            async_process: Some(false),
        };
        self.client.purchase_label(&req).await
    }

    pub async fn cancel_label(&self, transaction_id: &str) -> Result<super::client::RefundResponse, super::client::ShippoError> {
        let req = RefundRequest {
            transaction: transaction_id.to_string(),
        };
        self.client.request_refund(&req).await
    }
}
