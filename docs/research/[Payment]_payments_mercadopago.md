**Title**: Payment Processing Integration via Mercado Pago

**Problem Statement**:
Small businesses operating in Latin America (LATAM) need robust payment solutions tailored to local preferences. Stripe is not universally adopted or supported in all LATAM countries. Business owners need to accept local payment methods (like Pix in Brazil, Rapipago in Argentina) securely and reliably, without complex coding.

**Research Report**:
Mercado Pago is the leading payment processor in Latin America.
- **Ease of Use for Non-Technical Users**: Merchants can easily generate payment links or set up subscription plans directly from their Mercado Pago account without coding. For deeper integration, OHC can use Checkout Pro or Checkout Bricks.
- **Features**: Supports a wide array of local payment methods across AR, BR, CL, CO, MX, PE, UY. Offers Checkout Pro (ready-to-use redirect), Checkout Bricks (modular UI), and Subscription plans for recurring billing.
- **Reputation & Reliability**: Extremely high trust and reliability in the LATAM market; considered the standard e-commerce backbone in the region.
- **Pricing**: Standard payment processing fees apply based on the country and settlement speed selected by the merchant. No fixed monthly costs for the basic checkout APIs.
- **Cloud vs Standalone**: The API is purely HTTP/REST, so it works identically in Cloud and Standalone modes. Webhooks require a publicly accessible URL, so Standalone users might need a relay or polling mechanism if they are behind a NAT.

**Design Doc**:
- **Trigger**: User selects "Mercado Pago" as their payment provider in OHC settings and inputs their production credentials (Access Token).
- **Action**: When a customer checks out, OHC generates a preference via the Checkout API and redirects the user (Checkout Pro) or renders the Checkout Bricks UI.
- **User View**: The business owner sees a simple toggle to enable Mercado Pago. Customers in LATAM see localized payment options (e.g., Pix, Boletos, local credit cards) during checkout.
- **Architecture**: Implement the Mercado Pago SDK. Create endpoints to generate checkout preferences. Set up a secure webhook receiver to handle `payment.created` and `payment.updated` events to mark orders as paid in OHC.

**Implementation Prompt**:
Integrate Mercado Pago as an alternative payment provider for LATAM users. Implement the Checkout Pro flow for maximum simplicity and support for local payment methods. Provide a settings UI for merchants to input their Mercado Pago credentials safely. Ensure order status automatically updates to "Paid" when the Mercado Pago webhook confirms the transaction.

**Priority**: P2 (medium)
**Estimated Scope**: Medium
