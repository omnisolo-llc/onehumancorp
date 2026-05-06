# Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
While Stripe is the default payment processor for many platforms, it does not support all regions effectively. For small businesses operating in Latin America, local payment methods (like Pix in Brazil, or local credit card installments) are crucial for conversion. OHC needs an alternative payment provider to serve business owners in the LATAM market effectively.

## Research Report
**Findings & Data**: Mercado Pago is the leading digital payment platform in Latin America, deeply integrated into the local financial ecosystems of countries like Brazil, Argentina, Mexico, and Colombia.
**Ease of Use**: Familiar and highly trusted by consumers in LATAM. For the business owner, setting up an account is standard for their region. The integration into OHC will provide a standard checkout experience.
**Features**: Supports local credit cards, bank transfers, and highly popular instant payment systems like Pix.
**Pricing**: Standard transaction fees apply, which are competitive for the region. Settlement speeds vary but are generally standard for the industry.
**Reputation**: The dominant player in the LATAM e-commerce space.

## Design Doc
**Integration flow**:
1.  **Connection**: The business owner inputs their Mercado Pago API credentials (Access Token / Public Key) in the OHC payment settings.
2.  **Checkout Generation**: When a client needs to pay for an invoice or a booking in OHC, OHC generates a Mercado Pago "Preference" (checkout session).
3.  **Payment**: The client is redirected to the Mercado Pago hosted checkout page (or a transparent checkout modal) to complete the payment using local methods.
4.  **Confirmation**: Mercado Pago sends a webhook back to OHC to confirm the payment, marking the invoice/booking as paid in the OHC system.

## Implementation Prompt
**User-Facing Outcome**: A business owner in LATAM can choose Mercado Pago as their payment processor. When they send an invoice or require payment for a booking, their clients will see a Mercado Pago payment button and can pay using local methods like Pix or local credit cards.
**Acceptance Criteria**:
- UI to input Mercado Pago credentials.
- Ability to select Mercado Pago as the active payment provider instead of default options.
- Generation of functional Mercado Pago checkout links for invoices/bookings.
- Webhook listener to automatically update payment status in OHC upon successful transaction.
- Compatible with Cloud mode (via central webhooks) and Standalone mode (may require specific webhook URL configuration by the user).

## Priority
P1

## Estimated Scope
Medium
