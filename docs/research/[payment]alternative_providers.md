### Title
`[payment]alternative_providers`: Implement Mercado Pago Integration for LATAM

### Problem Statement
Stripe is fantastic, but it's not universally adopted or preferred in all markets. In regions like LATAM, local providers like Mercado Pago are essential for offering familiar payment methods (like Pix in Brazil or OXXO in Mexico). Without local options, business owners lose sales.

### Research Report
- **Tool**: Mercado Pago API
- **Pros**: Dominant market share in LATAM, supports local payment methods that Stripe misses.
- **Cons**: Documentation can be fragmented; API design is sometimes inconsistent compared to Stripe.
- **Reputation**: The undisputed leader in Latin American e-commerce payments.
- **Pricing**: Varies by country and payment method (typically 3-5% + fixed fee).
- **Ease of Use for Non-Technical Users**: Users link their existing Mercado Pago account. The checkout experience is highly trusted by local consumers.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: The business owner configures their Mercado Pago credentials in the "Payments" tab.
- **Action**: The OHC API server generates payment preferences and handles incoming IPN (Instant Payment Notification) webhooks to update order status.
- **User View**: Customers see Mercado Pago as a checkout option. Business owners see unified transaction logs alongside Stripe payments.

### Implementation Prompt
Add Mercado Pago as a supported payment gateway. This must include creating payment intents and handling IPN webhooks to confirm payment success. The checkout flow must support local payment methods relevant to the user's region. Ensure the system gracefully handles pending payments (e.g., waiting for cash deposits) and updates the order status asynchronously.

### Priority
P1

### Estimated Scope
Medium
