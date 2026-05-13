#[derive(Debug, Clone, PartialEq)]
pub enum MercadoPagoMethod {
    Pix,
    Boleto,
    CreditCard,
}

pub struct MercadoPagoRouter;

impl MercadoPagoRouter {
    pub fn select_method(amount: f64, country_code: &str) -> MercadoPagoMethod {
        if country_code == "BR" && amount < 1000.0 {
            MercadoPagoMethod::Pix
        } else if country_code == "AR" {
            MercadoPagoMethod::Boleto
        } else {
            MercadoPagoMethod::CreditCard
        }
    }
}
