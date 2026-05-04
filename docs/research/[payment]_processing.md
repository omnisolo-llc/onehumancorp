# [payment] Global Alternative Payment Processing Options

## Title
Expand Payment Processing Beyond Stripe

## Problem Statement
While Stripe is powerful, many small business owners operate in regions where Stripe is unsupported or where local payment methods dominate (e.g., PIX in Brazil, UPI in India, M-Pesa in Kenya). A user like Fatima (Food Cart Operator) needs a payment system that her local customers actually use, with fast settlement times and low fees. Relying solely on Stripe limits OHC's global reach and accessibility.

## Research Report
### Market Evaluation
- **Mercado Pago**: Dominant in Latin America. Supports local methods like PIX (Brazil) and OXXO (Mexico).
    - *Ease of use (for user)*: Familiar to LATAM users.
    - *Integration*: Robust API, but documentation is heavily regionalized.
    - *Cloud vs. Standalone*: Works in both. Webhooks require a public endpoint, making Standalone setup challenging without a tunneling service.
- **Razorpay / PayU**: Leading providers in India. Crucial for UPI payments.
    - *Ease of use (for user)*: Essential for the Indian market.
    - *Integration*: Good developer experience, handles complex Indian compliance.
    - *Cloud vs. Standalone*: Same webhook challenges as Mercado Pago for Standalone deployments.
- **PayPal / Braintree**: Globally recognized, strong consumer trust.
    - *Ease of use (for user)*: Easy connection, high conversion rates.
    - *Integration*: Complex legacy APIs, higher fees than Stripe in some regions.
- **Square**: Strong competitor for in-person POS.
    - *Ease of use (for user)*: Excellent hardware ecosystem.
    - *Integration*: Closed ecosystem, difficult to integrate tightly with an external platform like OHC without driving users to Square's own software.

### Integration Risks & Considerations
- **Unified Abstraction**: OHC needs a standardized payment interface (a "Payment Gateway" abstraction) so that the core operations (charge, refund, subscription, webhook handling) work seamlessly regardless of the underlying provider.
- **Regulatory Complexity**: Expanding payment providers means handling different KYC (Know Your Customer) flows and regional tax compliance.
- **Webhook Normalization**: Different providers have different webhook structures and security mechanisms (signatures vs. IP whitelisting).

## Design Doc
### User Experience
1. **Provider Selection**: In the "Finance & Payments" tab, the user clicks "Set up Payments." Based on their selected country, OHC recommends the best provider (e.g., Stripe for US, Mercado Pago for Brazil).
2. **Simplified Onboarding**: The user connects their account via OAuth or API keys, guided by a simple wizard.
3. **Unified Dashboard**: Regardless of the provider, all transactions, refunds, and payouts are displayed in the standardized OHC "Finance" dashboard. The "Accountant" AI agent analyzes this data uniformly.
4. **Checkout Experience**: The customer checkout flow dynamically presents the payment methods supported by the connected provider (e.g., showing a PIX QR code instead of a credit card form).

### System Flow
- OHC introduces a `PaymentProvider` interface in the backend.
- When a checkout session is initiated, OHC calls the specific provider implementation based on the tenant's configuration.
- The UI renders the provider-specific checkout UI (often a hosted page or a drop-in widget).
- Provider webhooks are received, verified, and mapped to standard OHC events (`payment_succeeded`, `payment_failed`).
- These standard events trigger the Operations and Finance AI agents to update orders and generate reports.

## Implementation Prompt
Design and implement a "Payment Gateway" abstraction layer that allows OHC to support multiple payment providers beyond Stripe, specifically targeting Mercado Pago for LATAM and Razorpay for India. Create a UI flow in the "Finance & Payments" department that suggests the appropriate provider based on the user's region and guides them through connection. Ensure that the core OHC transaction data model remains standardized, regardless of the underlying provider, so that AI reporting works seamlessly. Do not prescribe specific database schemas or API endpoints; focus on the abstraction architecture and the user onboarding flow.

## Priority
P2

## Estimated Scope
Large