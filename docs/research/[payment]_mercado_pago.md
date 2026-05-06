### Title
Native Integration of Local Payment Methods (Mercado Pago)

### Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

### Research Report
- **Strategy**: Direct API integration with Mercado Pago for seamless LATAM coverage.
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Standard transaction fees apply; merchants expect these.
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).

### Design Doc
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.
- **AI Integration**: Finance & Payments Agent seamlessly aggregates revenue across providers into a unified native dashboard.

### Implementation Prompt
Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings. Webhooks must normalize into standard OHC order fulfillment events.
- **Acceptance Criteria**: Merchant in a supported region can connect Mercado Pago natively. Customers can checkout using local methods. Orders are marked paid upon successful webhook receipt.
- **Priority**: P1
- **Estimated Scope**: Large
