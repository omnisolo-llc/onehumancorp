pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}

pub fn get_catalog() -> Vec<IntegrationProvider> {
    let mut catalog = vec![];

    let nats_provider = crate::integrations::nats::provider::NatsProvider::with_client(
        std::sync::Arc::new(crate::integrations::nats::client::RealNatsClient::dummy()),
        "nats://localhost:4222"
    ).into_integration_provider();
    catalog.push(nats_provider);

    catalog.push(crate::integrations::manychat::provider::ManychatProvider::new().into_integration_provider());
    catalog.push(crate::integrations::calendly::provider::CalendlyProvider::new().into_integration_provider());
    catalog.push(crate::integrations::mailerlite::provider::MailerliteProvider::new().into_integration_provider());
    catalog.push(crate::integrations::mercadopago::provider::MercadopagoProvider::new().into_integration_provider());
    catalog.push(crate::integrations::shippo::provider::ShippoProvider::new().into_integration_provider());
    catalog.push(crate::integrations::twilio::provider::TwilioProvider::new().into_integration_provider());
    catalog.push(crate::integrations::zoom::provider::ZoomProvider::new().into_integration_provider());

    catalog
}
