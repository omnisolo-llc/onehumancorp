# Payment Processing Integration

## Title
LATAM Payment Gateway Integration (Mercado Pago)

## Problem Statement
Small business owners in Latin America often cannot use Stripe due to regional restrictions or high local currency conversion fees. They need a localized, trusted payment processor that supports local payment methods (like Pix in Brazil or OXXO in Mexico) to successfully charge their customers.

## Research Report
*   **Target Tools:** Mercado Pago.
*   **Pros:** Dominant market share in LATAM. Supports highly specific local payment methods. High trust among LATAM consumers.
*   **Cons:** API documentation can be fragmented. Sandbox environments are sometimes less reliable than Stripe's.
*   **Ease of Use for Non-Technical Users:** High. The user connects their existing Mercado Pago account. Customers see a familiar checkout flow.
*   **Pricing:** Transaction fees vary significantly by country and payment method (e.g., credit card vs. local cash voucher).
*   **Cloud vs. Standalone:**
    *   *Cloud:* Works well via standard OAuth and webhooks.
    *   *Standalone:* Can function if the user inputs API keys, but webhook delivery for asynchronous payment confirmations (like Pix) requires a public endpoint.

## Design Doc
1.  **Connection Flow:** User navigates to Settings > Payments. They select "Mercado Pago" and log into their account to authorize the integration.
2.  **Checkout:** When an OHC invoice or booking requires payment, the customer is redirected to a Mercado Pago hosted checkout page (or an embedded form).
3.  **Confirmation:** Once paid, the OHC system updates the invoice status to "Paid".

## Implementation Prompt
Integrate Mercado Pago as an alternative payment provider to Stripe. Add a "Connect Mercado Pago" option in the payment settings. When generating an invoice or a paid booking link, allow the business owner to select Mercado Pago as the processor. The checkout experience must support local payment methods (e.g., Pix). Ensure that asynchronous payment confirmations (where the customer pays later, like OXXO) correctly update the OHC invoice status when the webhook is received.

## Priority
P1 (high)

## Estimated Scope
Large
