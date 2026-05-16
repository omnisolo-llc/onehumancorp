# Scout: Tool Integration Research

## Payment Processing
**Title**: Integrate Alipay for Cross-Border Chinese Consumers
**Problem Statement**: Businesses selling internationally or in regions with high Chinese tourism (like certain parts of SE Asia or Europe) miss out on significant sales because customers prefer to pay using their native Alipay wallets rather than international credit cards.
**Research Report**:
- Alipay is the dominant digital wallet in China. Supporting it is critical for businesses targeting Chinese expatriates, tourists, or cross-border e-commerce.
- The user experience is heavily QR-code based (for in-person) or seamless wallet redirects (online).
- Pricing: Transaction fees vary but are generally competitive with standard credit card processing for cross-border.
- Compatibility: Supported as a payment method via Stripe's API, but a direct integration via a local gateway (or Alipay Global) might be needed depending on the merchant's country. Cloud mode handles routing; Standalone mode uses direct API keys.
**Design Doc**:
- During onboarding, if the user indicates they sell internationally or operate in a relevant region, Alipay is suggested.
- In the storefront checkout, an "Alipay" option generates a QR code (desktop) or deep links to the Alipay app (mobile).
- For in-person point-of-sale features, the OHC dashboard displays an Alipay QR code for the customer to scan.
**Implementation Prompt**: Implement an Alipay payment method at checkout, optimized for both desktop (QR code generation) and mobile (deep linking). If using Stripe as the underlying aggregator, ensure the Alipay payment method is activated and appropriately handled in webhooks.
**Priority**: P2
**Estimated Scope**: Large