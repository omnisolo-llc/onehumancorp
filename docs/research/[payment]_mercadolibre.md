### Title
Integrate Mercado Pago for LATAM Payment Processing

### Problem Statement
The OHC platform defaults to Stripe for payment processing. However, small business owners operating in Latin America require localized payment methods that Stripe may not fully support or offer at competitive rates. They need a payment provider trusted by LATAM consumers to process online storefront orders and deposits without friction.

### Research Report
**Tool Evaluated:** Mercado Pago (Mercado Libre)
**Overview:** Mercado Libre, founded in 1999 in Argentina, is the largest e-commerce and payments ecosystem in Latin America. It reported $28.9 billion in revenue in 2025. Mercado Pago is its secure payment system, designed originally for the marketplace but widely used across the region as a standalone payment gateway.
**Key Features & Advantages:**
- Ubiquitous in LATAM (Argentina, Brazil, Mexico, Colombia, Chile, etc.).
- Supports local payment methods (cash payments at convenience stores, local credit cards with installment plans).
- High consumer trust in the region.
**Risks:** Development requires handling complex, region-specific payment flows (like asynchronous cash payments where the order is pending until paid at a physical store), which adds complexity compared to instant credit card approvals.
**Ease of Use:** Highly familiar to consumers in LATAM. The integration must be seamless for the business owner via OHC.
**Pricing:** Transaction-based pricing, competitive for the region.
**Deployment:** Cloud-native API integration.

### Design Doc
**Integration Trigger:** A business owner located in a supported LATAM country selects "Mercado Pago" in the "Finance & Payments" settings of OHC.
**Action:** The owner authenticates via OAuth. OHC sets Mercado Pago as the active payment gateway for their storefront checkout flow.
**User Experience:**
- **Business Owner:** Sees payments settle into their Mercado Pago account. OHC's Finance AI agent tracks these revenues identically to how it tracks Stripe payments.
- **Customer:** During checkout on the OHC storefront, they are presented with Mercado Pago's checkout UI, allowing them to use local installment plans or cash payment vouchers (e.g., Boleto in Brazil, PagoFácil in Argentina).

### Implementation Prompt
Implement a Mercado Pago payment gateway integration as an alternative to Stripe for LATAM-based merchants.

**Acceptance Criteria:**
1. Implement an OAuth connection flow for merchants to link their Mercado Pago accounts.
2. Build a checkout integration that redirects the customer to Mercado Pago's secure checkout flow and handles the return redirect.
3. Implement a webhook handler to listen for asynchronous payment status updates (e.g., when a customer pays a cash voucher 2 days later) and update the OHC order status from "Pending" to "Paid".
4. Ensure the Finance & Payments dashboard aggregates revenue data from Mercado Pago alongside any other payment methods.

### Priority
P1

### Estimated Scope
Large
