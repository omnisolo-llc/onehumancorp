# Alternative Global Payment Gateways

**Problem Statement**: Stripe isn't supported everywhere, or fees are too high for local transactions (e.g., LATAM, India).

**Research Report**: Mercado Pago (LATAM), Paytm (India), and Alipay (China) are critical for global adoption. They offer lower local fees and preferred payment methods. APIs vary wildly. Standalone is viable if users plug in their own credentials.

**Design Doc**: Pluggable payment gateway architecture. User selects region/provider and enters credentials. Checkout UI adapts to provider.

**Implementation Prompt**: Implement support for Mercado Pago and Paytm alongside Stripe for checkout.

**Priority**: P1
**Estimated Scope**: Large
