use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuickBooksAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuickBooksSalesReceipt {
    pub id: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuickBooksInvoice {
    pub id: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuickBooksRefundReceipt {
    pub id: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuickBooksPayment {
    pub id: String,
    pub total_amount: f64,
}


pub struct QuickBooksClient {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
}

impl QuickBooksClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        QuickBooksClient {
            client_id,
            client_secret,
            access_token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub async fn get_auth_url(&self, redirect_uri: &str) -> Result<String, String> {
        Ok(format!(
            "https://appcenter.intuit.com/connect/oauth2?client_id={}&redirect_uri={}&response_type=code&scope=com.intuit.quickbooks.accounting",
            self.client_id, redirect_uri
        ))
    }

    pub async fn exchange_code_for_token(&self, _code: &str, _redirect_uri: &str) -> Result<QuickBooksAuthToken, String> {
        // Mock token exchange
        Ok(QuickBooksAuthToken {
            access_token: "mock_qb_access_token".to_string(),
            refresh_token: "mock_qb_refresh_token".to_string(),
            expires_in: 3600,
        })
    }

    pub async fn create_sales_receipt(&self, amount: f64, _customer_name: &str) -> Result<QuickBooksSalesReceipt, String> {
        if self.access_token.is_none() {
            return Err("Missing access token".to_string());
        }

        Ok(QuickBooksSalesReceipt {
            id: "qb_sr_123".to_string(),
            total_amount: amount,
        })
    }

    pub async fn create_invoice(&self, amount: f64, _customer_name: &str) -> Result<QuickBooksInvoice, String> {
        if self.access_token.is_none() {
            return Err("Missing access token".to_string());
        }

        Ok(QuickBooksInvoice {
            id: "qb_inv_123".to_string(),
            total_amount: amount,
        })
    }

    pub async fn create_refund_receipt(&self, amount: f64, _customer_name: &str) -> Result<QuickBooksRefundReceipt, String> {
        if self.access_token.is_none() {
            return Err("Missing access token".to_string());
        }

        Ok(QuickBooksRefundReceipt {
            id: "qb_rr_123".to_string(),
            total_amount: amount,
        })
    }

    pub async fn create_payment(&self, amount: f64, _customer_name: &str) -> Result<QuickBooksPayment, String> {
        if self.access_token.is_none() {
            return Err("Missing access token".to_string());
        }

        Ok(QuickBooksPayment {
            id: "qb_pmt_123".to_string(),
            total_amount: amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quickbooks_client() {
        let client = QuickBooksClient::new("client_id".to_string(), "client_secret".to_string());
        assert_eq!(client.access_token, None);

        let url = client.get_auth_url("http://localhost/callback").await.unwrap();
        assert!(url.contains("client_id=client_id"));

        let token = client.exchange_code_for_token("auth_code", "http://localhost/callback").await.unwrap();
        assert_eq!(token.access_token, "mock_qb_access_token");

        let authed_client = client.with_token(token.access_token);

        let receipt = authed_client.create_sales_receipt(100.0, "Test").await.unwrap();
        assert_eq!(receipt.total_amount, 100.0);
    }
}
