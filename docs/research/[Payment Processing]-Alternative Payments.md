# Global Alternative Payment Methods Support

## Problem Statement
Not all customers use Stripe or standard credit cards. Business owners in specific regions (LATAM, India, China) lose sales because they don't support local payment methods like Mercado Pago, Paytm, or Alipay.

## Research Report
Evaluated alternative payment providers for specific markets.

- **Ease of Use**: Crucial for international or localized businesses to increase conversion rates.
- **Pricing**: Transaction fees vary (typically 1-3%), but setup costs should be minimal.
- **Risks**: Settlement delays, varied currency support, handling disparate webhook failure modes.
- **Modes**: Fully compatible with both Cloud and Standalone environments.

## Design Doc
During checkout setup, the business owner can enable regional payment providers. OHC routes checkout requests to the selected provider's API. Webhooks confirm payment success, triggering order fulfillment workflows.

## Implementation Prompt
Extend the checkout configuration to allow enabling additional payment providers. Update the checkout page to dynamically render payment buttons based on the enabled providers and handle their respective redirect or modal flows.

## Priority
P2

## Estimated Scope
Large
