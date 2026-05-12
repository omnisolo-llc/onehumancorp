# Payment Processing Research Brief

## Title
Global and Localized Payment Processing

## Problem Statement
Getting paid is the lifeblood of any business. Relying solely on cash or manual bank transfers limits growth. However, many global small businesses cannot use Stripe due to geographic restrictions. They need a seamless way to accept credit cards and local payment methods (e.g., PIX, UPI) directly through their invoices or booking pages.

## Research Report
### Market Context
While Stripe dominates North America and Europe, local payment processors dominate emerging markets. A single payment integration is insufficient for a global platform like OHC.

### Tool Evaluations

#### 1. Stripe
- **Ease of Use:** Very high.
- **Pricing:** 2.9% + 30¢ per successful card charge.
- **Capabilities:** Massive global reach, excellent APIs, handles recurring billing well.
- **Reputation:** The gold standard, but not available in many LATAM/African/Asian countries.

#### 2. Mercado Pago
- **Ease of Use:** High for LATAM users.
- **Pricing:** Varies by country, generally higher per-transaction fees than Stripe.
- **Capabilities:** Supports local methods like PIX in Brazil, OXXO in Mexico.
- **Reputation:** Essential for doing business in Latin America.

#### 3. Razorpay
- **Ease of Use:** High for Indian businesses.
- **Pricing:** ~2% per transaction.
- **Capabilities:** Deep support for UPI, RuPay, and local wallets.
- **Reputation:** The Stripe of India.

### Recommended Direction
Design a modular payment gateway interface in OHC. Start with Stripe for immediate coverage in Western markets, but build the architecture to easily plug in Mercado Pago and Razorpay next.

## Design Doc
### Trigger & Action
1. **Trigger:** Customer views an OHC-generated invoice and clicks "Pay Now."
2. **Action:** Customer is redirected to a hosted checkout page (e.g., Stripe Checkout) or a local modal. Upon successful payment, a webhook notifies OHC to mark the invoice as "Paid."
3. **User View:** The business owner sees their connected payment gateways in settings. Invoices automatically update their status, and funds are routed to the owner's bank account.

### Environment Support
- **Cloud Mode:** OHC acts as the platform (e.g., Stripe Connect), facilitating payments and potentially taking an application fee.
- **Standalone Mode:** The user connects their direct API keys for Stripe/Mercado Pago. Webhooks must be handled via polling or local tunneling.

## Implementation Prompt
Build a "Payment Integration" module starting with Stripe Checkout.
- Allow the business owner to input their Stripe API keys (or connect via OAuth).
- Generate a unique payment link for invoices.
- When a customer pays via the link, the system must securely handle the webhook and update the invoice status from "Pending" to "Paid."
- Do not store credit card data directly in the database.
- Acceptance criteria: Successfully process a test-mode payment via Stripe and verify the invoice status updates automatically.

## Priority
P0 (Critical)

## Estimated Scope
Medium

### Extended Payment Architecture Analysis
#### PCI Compliance and Security
Direct handling of cardholder data is heavily restricted. The integration must strictly utilize hosted checkout pages or tokenization methods (such as Stripe Elements) so that PAN (Primary Account Number) data never touches OHC servers. This guarantees PCI-DSS compliance is outsourced to the gateway.

#### Idempotency and Webhook Reliability
Network failures during payment execution are critical errors. Webhooks handling payment confirmations must be strictly idempotent to prevent double-crediting invoices if the gateway sends duplicate events. Retries must be managed efficiently, and a reconciliation job should occasionally poll the gateway for missed status updates.

#### Multi-Currency and Local Methods
A global payment interface requires robust currency handling. The data model must store exact amounts alongside their respective ISO 4217 currency codes. Furthermore, alternative methods like Bank Transfers, SEPA, or iDEAL require asynchronous confirmation flows—an invoice might remain "Pending Verification" for days before transitioning to "Paid".

#### Refund Lifecycle
Handling refunds is notoriously complex. If a business owner initiates a refund in OHC, the system must communicate with the gateway, track the refund state, and update the invoice accordingly. Partial refunds must also be supported.

### User Persona Match
- **Fatima (Boutique Owner):** High value. Needs to process credit cards seamlessly to convert online window shoppers.
- **Carlos (Consultant):** High value. Requires clients to pre-pay for consultations or settle large monthly retainers electronically.

### Conclusion
A flawless payment experience is the cornerstone of business operations. By providing a secure, reliable, and eventually localized gateway interface, OHC immediately justifies its value proposition to the business owner.
