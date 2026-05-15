# Alternative Global Payment Gateways

## Problem Statement
Stripe isn't available everywhere, and its fees can be high for micro-transactions. Small businesses in emerging markets need local payment options (e.g., Pix in Brazil, UPI in India) to accept money from their customers.

## Research Report
**Competitive Landscape:**
1. **Mercado Pago:** Dominant in LATAM. Essential for Brazil, Argentina, Mexico.
2. **Razorpay / Paytm:** Dominant in India. Required for UPI support.
3. **Stripe:** Great for US/EU, but lacks penetration in some regions.

**Evaluation:**
- **Ease of Use:** Must handle currency conversion and local tax compliance smoothly.
- **Failure Rates:** Alternative methods often have different failure modes (e.g., delayed confirmation for bank transfers).
- **Cloud vs Standalone:** Both can integrate via API, but webhook handling in Standalone requires a reliable tunnel or polling mechanism.

## Design Doc
- **Trigger:** User selects their region during onboarding, which unlocks specific payment providers.
- **Action:** User connects their local gateway (e.g., Mercado Pago). OHC generates payment links or checkout pages using that provider.
- **User Experience:** A seamless checkout for the end-customer using familiar local payment methods.

## Implementation Prompt
Implement an extensible payment provider interface. Add support for Mercado Pago alongside the existing Stripe integration. The business owner should simply select 'Enable Mercado Pago' and paste their API keys. The checkout UI should dynamically display the correct payment elements based on the active provider. Ensure robust handling of asynchronous payment confirmation webhooks.

## Priority
P0

## Estimated Scope
Large
