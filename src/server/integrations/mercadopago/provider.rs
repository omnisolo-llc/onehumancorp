pub struct MercadoPagoProvider {
    pub api_token: String,
}

impl MercadoPagoProvider {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }
}
