**Title**: Global Payment Processing (Mercado Pago, Paytm, Alipay)
**Problem Statement**: While Stripe is great, it doesn't cover every market effectively. A merchant in Brazil (Mercado Pago), India (Paytm), or China (Alipay) needs to accept payments using the methods their local customers actually use and trust, otherwise they lose sales at checkout.
**Research Report**:
- **Mercado Pago**: Operated by MercadoLibre, it is massive in Latin America. As of 2016, MercadoLibre had over 174 million users in LATAM. Essential for Brazil, Argentina, and Mexico.
- **Paytm**: Founded in 2010, it is an Indian multinational financial technology company specializing in digital payments. It is ubiquitous in India for QR code payments and mobile wallets.
- **Alipay**: Established in 2004 by Alibaba. It is a third-party mobile and online payment platform that is absolutely necessary for merchants targeting Chinese consumers.
- **Ease of Use**: High for the end consumer (usually scanning a QR code or entering a wallet PIN). Setup for the merchant involves standard API key generation.
- **Pricing**: Varies by region, but generally competitive with or cheaper than standard credit card interchange fees.
- **Reputation**: These are the dominant, most trusted financial apps in their respective regions.
- **Cloud/Standalone**: API calls for payment intent creation and webhooks for success verification work identically in Cloud and Standalone (with proper webhook tunneling).
**Design Doc**:
- **Trigger**: A customer in India proceeds to checkout on an OHC storefront.
- **Action**: The OHC checkout dynamically offers "Paytm" as a payment method based on the merchant's configured location/currency. OHC generates a Paytm transaction token.
- **UI**: The OHC Payments settings page should have a "Region Specific Providers" section. The merchant selects their provider from a dropdown and inputs their API credentials. The storefront checkout must gracefully display the appropriate button (e.g., a blue Alipay button or a QR code for Paytm).
**Implementation Prompt**: Architect a modular payment gateway interface in OHC. Implement adapters for Mercado Pago, Paytm, and Alipay. Ensure the checkout UI dynamically renders the correct payment buttons based on the merchant's configured active gateways. All webhook handlers for these providers must securely verify cryptographic signatures before marking an order as 'paid'.
**Priority**: P1
**Estimated Scope**: Large
