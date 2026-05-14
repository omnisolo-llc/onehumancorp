## 4. Payment Processing
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Stripe is not available or preferred in many Latin American countries. Businesses in these regions need a localized payment processor that supports local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- **Tool**: Mercado Pago API
- **Problem it solves for which persona**: Allows LATAM-based small businesses to accept online payments securely using methods their customers trust.
- **Ease of Use**: Familiar to the target market. Owner connects their Mercado Pago account via OAuth.
- **Pricing**: Varies by country, typically ~3-5% + flat fee. No monthly fixed costs.
- **Key Advantages**: Massive market share in LATAM; supports local cash-based and instant transfer methods.
- **Integration Risks**: Complex webhook verification; documentation can be fragmented.
- **Environment**: Cloud and Standalone supported.
**Design Doc**:
- **Trigger**: Customer clicks "Pay" on an OHC invoice.
- **Action**: OHC redirects to a Mercado Pago Checkout Pro link or renders a Web Tokenized Checkout.
- **User Interface**: Owner selects "Mercado Pago" as their payment provider in settings. Customers see localized payment options.
**Implementation Prompt**: Implement an alternative payment provider module using Mercado Pago. Generate Checkout Pro preference links for invoices and handle incoming webhooks to mark OHC invoices as paid.
**Priority**: P1
**Estimated Scope**: Medium
