use crate::integrations::mercadopago::client::MercadoPagoClient;

pub struct MercadoPagoProvider {
    #[allow(dead_code)]
    client: MercadoPagoClient,
}

impl MercadoPagoProvider {
    pub fn new(access_token: String) -> Self {
        Self {
            client: MercadoPagoClient::new(access_token),
        }
    }
}
